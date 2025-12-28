use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ergo_runtime::runtime::ExecutionContext as RuntimeExecutionContext;
use serde::{Deserialize, Serialize};

pub mod capture;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GraphId(String);

impl GraphId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(String);

impl EventId {
    pub fn new(id: impl Into<String>) -> Self {
        EventId(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrKind {
    NetworkTimeout,
    AdapterUnavailable,
    ValidationFailed,
    RuntimeError,
    DeadlineExceeded,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunTermination {
    Completed,
    TimedOut,
    Aborted,
    Failed(ErrKind),
}

/// ExecutionContext is intentionally opaque to non-adapter callers.
/// Its internals are owned by the runtime and are not constructible
/// outside this crate to satisfy CXT-1.
///
/// ```compile_fail
/// use ergo_adapter::ExecutionContext;
/// use ergo_runtime::runtime::ExecutionContext as RuntimeExecutionContext;
/// use std::collections::HashMap;
///
/// // Constructor is not visible outside ergo-adapter.
/// let runtime_ctx = RuntimeExecutionContext { trigger_state: HashMap::new() };
/// let _ctx = ExecutionContext::new(runtime_ctx);
/// ```
///
/// ```compile_fail
/// use ergo_adapter::ExecutionContext;
/// use ergo_runtime::runtime::ExecutionContext as RuntimeExecutionContext;
/// use std::collections::HashMap;
///
/// // Opaque fields cannot be set directly.
/// let runtime_ctx = RuntimeExecutionContext { trigger_state: HashMap::new() };
/// let _ctx = ExecutionContext { inner: runtime_ctx };
/// ```
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    inner: RuntimeExecutionContext,
}

impl ExecutionContext {
    pub(crate) fn new(inner: RuntimeExecutionContext) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &RuntimeExecutionContext {
        &self.inner
    }
}

/// Opaque absolute time used for deterministic scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventTime(Duration);

impl EventTime {
    pub fn from_duration(duration: Duration) -> Self {
        Self(duration)
    }

    pub fn as_duration(&self) -> Duration {
        self.0
    }

    pub fn saturating_add(&self, duration: Duration) -> Self {
        Self(self.0.saturating_add(duration))
    }
}

impl From<Duration> for EventTime {
    fn from(value: Duration) -> Self {
        EventTime::from_duration(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventPayload {
    pub data: Vec<u8>,
}

impl Default for EventPayload {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalEventKind {
    Tick,
    DataAvailable,
    Command,
}

#[derive(Debug, Clone)]
pub struct ExternalEvent {
    event_id: EventId,
    kind: ExternalEventKind,
    context: ExecutionContext,
    at: EventTime,
    payload: EventPayload,
}

impl ExternalEvent {
    pub(crate) fn new(
        event_id: EventId,
        kind: ExternalEventKind,
        context: ExecutionContext,
        at: EventTime,
        payload: EventPayload,
    ) -> Self {
        Self {
            event_id,
            kind,
            context,
            at,
            payload,
        }
    }

    pub fn mechanical_at(event_id: EventId, kind: ExternalEventKind, at: EventTime) -> Self {
        let runtime_ctx = RuntimeExecutionContext {
            trigger_state: HashMap::new(),
        };
        let context = ExecutionContext::new(runtime_ctx);
        Self::new(event_id, kind, context, at, EventPayload::default())
    }

    pub fn mechanical(event_id: EventId, kind: ExternalEventKind) -> Self {
        Self::mechanical_at(event_id, kind, EventTime::default())
    }

    pub fn with_payload(
        event_id: EventId,
        kind: ExternalEventKind,
        at: EventTime,
        payload: EventPayload,
    ) -> Self {
        let runtime_ctx = RuntimeExecutionContext {
            trigger_state: HashMap::new(),
        };
        let context = ExecutionContext::new(runtime_ctx);
        Self::new(event_id, kind, context, at, payload)
    }

    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    pub fn kind(&self) -> ExternalEventKind {
        self.kind
    }

    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn at(&self) -> EventTime {
        self.at
    }

    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuntimeHandle;

impl RuntimeHandle {
    pub fn run(
        &self,
        graph_id: &GraphId,
        event_id: &EventId,
        ctx: &ExecutionContext,
        deadline: Option<Duration>,
    ) -> RunTermination {
        let _ = graph_id;
        let _ = event_id;
        let _ = ctx.inner();
        if matches!(deadline, Some(d) if d.is_zero()) {
            return RunTermination::Aborted;
        }
        RunTermination::Completed
    }
}

pub trait RuntimeInvoker {
    fn run(
        &self,
        graph_id: &GraphId,
        event_id: &EventId,
        ctx: &ExecutionContext,
        deadline: Option<Duration>,
    ) -> RunTermination;
}

impl RuntimeInvoker for RuntimeHandle {
    fn run(
        &self,
        graph_id: &GraphId,
        event_id: &EventId,
        ctx: &ExecutionContext,
        deadline: Option<Duration>,
    ) -> RunTermination {
        Self::run(self, graph_id, event_id, ctx, deadline)
    }
}

#[derive(Clone)]
pub struct FaultRuntimeHandle {
    schedule: Arc<Mutex<HashMap<EventId, Vec<RunTermination>>>>,
    default: RunTermination,
}

impl Default for FaultRuntimeHandle {
    fn default() -> Self {
        Self::new(RunTermination::Completed)
    }
}

impl FaultRuntimeHandle {
    pub fn new(default: RunTermination) -> Self {
        Self {
            schedule: Arc::new(Mutex::new(HashMap::new())),
            default,
        }
    }

    pub fn with_schedule(
        default: RunTermination,
        schedule: HashMap<EventId, Vec<RunTermination>>,
    ) -> Self {
        Self {
            schedule: Arc::new(Mutex::new(schedule)),
            default,
        }
    }

    pub fn push_outcomes(&self, event_id: EventId, outcomes: Vec<RunTermination>) {
        let mut guard = self.schedule.lock().expect("fault schedule poisoned");
        guard.insert(event_id, outcomes);
    }
}

impl RuntimeInvoker for FaultRuntimeHandle {
    fn run(
        &self,
        graph_id: &GraphId,
        event_id: &EventId,
        ctx: &ExecutionContext,
        deadline: Option<Duration>,
    ) -> RunTermination {
        let _ = graph_id;
        let _ = ctx.inner();

        if matches!(deadline, Some(d) if d.is_zero()) {
            return RunTermination::Aborted;
        }

        let mut guard = self.schedule.lock().expect("fault schedule poisoned");
        let queue = guard.entry(event_id.clone()).or_default();
        if !queue.is_empty() {
            return queue.remove(0);
        }

        self.default.clone()
    }
}
