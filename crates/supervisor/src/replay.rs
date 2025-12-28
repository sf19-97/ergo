use std::sync::{Arc, Mutex};

use ergo_adapter::capture::ExternalEventRecord;
use ergo_adapter::RuntimeInvoker;

use crate::{CaptureBundle, DecisionLog, DecisionLogEntry, EpisodeInvocationRecord, Supervisor};

#[derive(Clone, Default)]
pub struct MemoryDecisionLog {
    entries: Arc<Mutex<Vec<DecisionLogEntry>>>,
}

impl DecisionLog for MemoryDecisionLog {
    fn log(&self, entry: DecisionLogEntry) {
        let mut guard = self.entries.lock().expect("decision log poisoned");
        guard.push(entry);
    }
}

impl MemoryDecisionLog {
    pub fn records(&self) -> Vec<EpisodeInvocationRecord> {
        let guard = self.entries.lock().expect("decision log poisoned");
        guard.iter().map(EpisodeInvocationRecord::from).collect()
    }
}

pub fn replay<R: RuntimeInvoker + Clone>(
    bundle: &CaptureBundle,
    runtime: R,
) -> Vec<EpisodeInvocationRecord> {
    let decision_log = MemoryDecisionLog::default();
    let mut supervisor = Supervisor::with_runtime(
        bundle.graph_id.clone(),
        bundle.config.clone(),
        decision_log.clone(),
        runtime,
    );

    for record in &bundle.events {
        supervisor.on_event(rehydrate_event(record));
    }

    decision_log.records()
}

fn rehydrate_event(record: &ExternalEventRecord) -> ergo_adapter::ExternalEvent {
    record.rehydrate()
}
