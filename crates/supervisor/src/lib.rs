use std::collections::VecDeque;
use std::time::Duration;

use ergo_adapter::{ErrKind, EventTime, ExternalEvent, GraphId, RunTermination, RuntimeHandle};

/// SUP-7: DecisionLog is write-only. No read/query surface is ever exposed.
pub trait DecisionLog {
    fn log(&self, entry: DecisionLogEntry);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DeterministicClock {
    now: EventTime,
}

impl DeterministicClock {
    fn new() -> Self {
        Self {
            now: EventTime::default(),
        }
    }

    fn advance_to(&mut self, at: EventTime) {
        if at > self.now {
            self.now = at;
        }
    }

    fn now(&self) -> EventTime {
        self.now
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EpisodeId(u64);

impl EpisodeId {
    pub fn new(id: u64) -> Self {
        EpisodeId(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Invoke,
    Skip,
    Defer,
}

#[derive(Debug, Clone)]
pub struct DecisionLogEntry {
    pub graph_id: GraphId,
    pub event: ExternalEvent,
    pub decision: Decision,
    pub scheduled_for: Option<Duration>,
    pub episode_id: EpisodeId,
    pub termination: RunTermination,
}

#[derive(Debug, Clone, Default)]
pub struct Constraints {
    pub max_in_flight: Option<usize>,
    pub max_per_window: Option<usize>,
    pub rate_window: Option<Duration>,
    pub deadline: Option<Duration>,
    pub max_retries: usize,
}

pub struct Supervisor<L: DecisionLog> {
    graph_id: GraphId,
    constraints: Constraints,
    decision_log: L,
    runtime: RuntimeHandle,
    next_episode_id: u64,
    in_flight: usize,
    recent_invocations: VecDeque<EventTime>,
    clock: DeterministicClock,
}

impl<L: DecisionLog> Supervisor<L> {
    pub fn new(graph_id: GraphId, constraints: Constraints, decision_log: L) -> Self {
        Self {
            graph_id,
            constraints,
            decision_log,
            runtime: RuntimeHandle::default(),
            next_episode_id: 0,
            in_flight: 0,
            recent_invocations: VecDeque::new(),
            clock: DeterministicClock::new(),
        }
    }

    pub fn on_event(&mut self, event: ExternalEvent) {
        self.clock.advance_to(event.at());
        let now = self.clock.now();
        let episode_id = self.next_episode_id();

        if self.is_concurrency_saturated() {
            self.log_decision(
                event,
                Decision::Defer,
                Some(Duration::ZERO),
                episode_id,
                RunTermination::Aborted,
            );
            return;
        }

        if let Some(delay) = self.rate_limit_delay(now) {
            self.log_decision(
                event,
                Decision::Defer,
                Some(delay),
                episode_id,
                RunTermination::Aborted,
            );
            return;
        }

        self.in_flight = self.in_flight.saturating_add(1);
        if self.constraints.max_per_window.is_some() && self.constraints.rate_window.is_some() {
            self.recent_invocations.push_back(now);
        }

        let termination = self.invoke_with_retries(event.context());

        self.in_flight = self.in_flight.saturating_sub(1);

        self.log_decision(event, Decision::Invoke, None, episode_id, termination);
    }

    fn next_episode_id(&mut self) -> EpisodeId {
        let id = EpisodeId::new(self.next_episode_id);
        self.next_episode_id = self.next_episode_id.saturating_add(1);
        id
    }

    fn is_concurrency_saturated(&self) -> bool {
        matches!(self.constraints.max_in_flight, Some(max) if self.in_flight >= max)
    }

    fn rate_limit_delay(&mut self, now: EventTime) -> Option<Duration> {
        let Some(max_per_window) = self.constraints.max_per_window else {
            return None;
        };
        let Some(window) = self.constraints.rate_window else {
            return None;
        };

        while let Some(front) = self.recent_invocations.front() {
            if now.as_duration().saturating_sub(front.as_duration()) >= window {
                self.recent_invocations.pop_front();
            } else {
                break;
            }
        }

        if self.recent_invocations.len() >= max_per_window {
            if let Some(front) = self.recent_invocations.front() {
                let elapsed = now.as_duration().saturating_sub(front.as_duration());
                let delay = window.saturating_sub(elapsed);
                return Some(delay);
            }
        }

        None
    }

    fn invoke_with_retries(&self, ctx: &ergo_adapter::ExecutionContext) -> RunTermination {
        let mut attempts = 0_usize;
        let mut termination = self
            .runtime
            .run(&self.graph_id, ctx, self.constraints.deadline);

        while attempts < self.constraints.max_retries && Self::should_retry(&termination) {
            attempts = attempts.saturating_add(1);
            termination = self
                .runtime
                .run(&self.graph_id, ctx, self.constraints.deadline);
        }

        termination
    }

    fn should_retry(termination: &RunTermination) -> bool {
        match termination {
            RunTermination::Failed(err) => matches!(
                err,
                ErrKind::NetworkTimeout | ErrKind::AdapterUnavailable | ErrKind::RuntimeError
            ),
            RunTermination::TimedOut => true,
            _ => false,
        }
    }

    fn log_decision(
        &self,
        event: ExternalEvent,
        decision: Decision,
        scheduled_for: Option<Duration>,
        episode_id: EpisodeId,
        termination: RunTermination,
    ) {
        let entry = DecisionLogEntry {
            graph_id: self.graph_id.clone(),
            event,
            decision,
            scheduled_for,
            episode_id,
            termination,
        };
        self.decision_log.log(entry);
    }
}
