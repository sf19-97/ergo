use std::collections::HashMap;
use std::time::Duration;

use ergo_runtime::runtime::ExecutionContext as RuntimeExecutionContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphId(String);

impl GraphId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrKind {
    NetworkTimeout,
    AdapterUnavailable,
    ValidationFailed,
    RuntimeError,
    DeadlineExceeded,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct EventTime(Duration);

impl EventTime {
    pub fn from_duration(duration: Duration) -> Self {
        Self(duration)
    }

    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

impl From<Duration> for EventTime {
    fn from(value: Duration) -> Self {
        EventTime::from_duration(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalEventKind {
    Tick,
    DataAvailable,
    Command,
}

#[derive(Debug, Clone)]
pub struct ExternalEvent {
    kind: ExternalEventKind,
    context: ExecutionContext,
    at: EventTime,
}

impl ExternalEvent {
    pub(crate) fn new(kind: ExternalEventKind, context: ExecutionContext, at: EventTime) -> Self {
        Self { kind, context, at }
    }

    pub fn mechanical_at(kind: ExternalEventKind, at: EventTime) -> Self {
        let runtime_ctx = RuntimeExecutionContext {
            trigger_state: HashMap::new(),
        };
        let context = ExecutionContext::new(runtime_ctx);
        Self::new(kind, context, at)
    }

    pub fn mechanical(kind: ExternalEventKind) -> Self {
        Self::mechanical_at(kind, EventTime::default())
    }

    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    pub fn kind(&self) -> ExternalEventKind {
        self.kind
    }

    pub fn at(&self) -> EventTime {
        self.at
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuntimeHandle;

impl RuntimeHandle {
    pub fn run(
        &self,
        graph_id: &GraphId,
        ctx: &ExecutionContext,
        deadline: Option<Duration>,
    ) -> RunTermination {
        let _ = graph_id;
        let _ = ctx.inner();
        if matches!(deadline, Some(d) if d.is_zero()) {
            return RunTermination::Aborted;
        }
        RunTermination::Completed
    }
}
