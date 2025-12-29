//! Integration tests for Supervisor with real RuntimeHandle execution path.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ergo_adapter::{EventId, ExternalEvent, ExternalEventKind, GraphId, RunTermination};
use ergo_runtime::catalog::{build_core_catalog, core_registries};
use ergo_runtime::cluster::{
    ExpandedEdge, ExpandedEndpoint, ExpandedGraph, ExpandedNode, ImplementationInstance,
    OutputPortSpec, OutputRef, ParameterValue,
};
use ergo_supervisor::{Constraints, Decision, DecisionLog, DecisionLogEntry, Supervisor};

/// Test-only DecisionLog that captures entries for verification.
#[derive(Clone)]
struct CapturingLog {
    entries: Arc<Mutex<Vec<DecisionLogEntry>>>,
}

impl CapturingLog {
    fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn entries(&self) -> Vec<DecisionLogEntry> {
        self.entries.lock().unwrap().clone()
    }
}

impl DecisionLog for CapturingLog {
    fn log(&self, entry: DecisionLogEntry) {
        self.entries.lock().unwrap().push(entry);
    }
}

/// Builds the canonical hello-world graph used in runtime tests.
/// Structure: number_source(3.0) -> gt <- number_source(1.0)
///            gt:result -> emit_if_true:input
///            emit_if_true:event -> ack_action:event
/// Since 3.0 > 1.0, trigger emits, action executes with outcome Filled.
fn build_hello_world_graph() -> ExpandedGraph {
    let mut nodes = HashMap::new();

    nodes.insert(
        "src_a".to_string(),
        ExpandedNode {
            runtime_id: "src_a".to_string(),
            authoring_path: vec![],
            implementation: ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([("value".to_string(), ParameterValue::Number(3.0))]),
        },
    );

    nodes.insert(
        "src_b".to_string(),
        ExpandedNode {
            runtime_id: "src_b".to_string(),
            authoring_path: vec![],
            implementation: ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([("value".to_string(), ParameterValue::Number(1.0))]),
        },
    );

    nodes.insert(
        "gt1".to_string(),
        ExpandedNode {
            runtime_id: "gt1".to_string(),
            authoring_path: vec![],
            implementation: ImplementationInstance {
                impl_id: "gt".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::new(),
        },
    );

    nodes.insert(
        "emit".to_string(),
        ExpandedNode {
            runtime_id: "emit".to_string(),
            authoring_path: vec![],
            implementation: ImplementationInstance {
                impl_id: "emit_if_true".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::new(),
        },
    );

    nodes.insert(
        "act".to_string(),
        ExpandedNode {
            runtime_id: "act".to_string(),
            authoring_path: vec![],
            implementation: ImplementationInstance {
                impl_id: "ack_action".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([("accept".to_string(), ParameterValue::Bool(true))]),
        },
    );

    let edges = vec![
        ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src_a".to_string(),
                port_name: "value".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "a".to_string(),
            },
        },
        ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src_b".to_string(),
                port_name: "value".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "b".to_string(),
            },
        },
        ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "result".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "emit".to_string(),
                port_name: "input".to_string(),
            },
        },
        ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "emit".to_string(),
                port_name: "event".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "act".to_string(),
                port_name: "event".to_string(),
            },
        },
    ];

    ExpandedGraph {
        nodes,
        edges,
        boundary_inputs: Vec::new(),
        boundary_outputs: vec![OutputPortSpec {
            name: "action_outcome".to_string(),
            maps_to: OutputRef {
                node_id: "act".to_string(),
                port_name: "outcome".to_string(),
            },
        }],
    }
}

/// SUP-2 verification: Supervisor::new() -> RuntimeHandle::new() -> runtime::run() -> Completed
///
/// This test proves the full execution path works:
/// 1. Supervisor::new() constructs a real RuntimeHandle (not a test double)
/// 2. RuntimeHandle::run() calls ergo_runtime::runtime::run()
/// 3. The graph executes successfully
/// 4. RuntimeHandle returns RunTermination::Completed
/// 5. Supervisor logs Decision::Invoke with termination: Some(Completed)
#[test]
fn supervisor_with_real_runtime_executes_hello_world() {
    // Build the hello-world graph
    let graph = Arc::new(build_hello_world_graph());

    // Build catalog and registries using the core implementations
    let catalog = Arc::new(build_core_catalog());
    let registries = Arc::new(core_registries().expect("core registries should build"));

    // Create capturing log to verify decisions
    let log = CapturingLog::new();

    // Construct Supervisor using Supervisor::new() — NOT with_runtime()
    // This uses the real RuntimeHandle, proving the full execution path
    let mut supervisor = Supervisor::new(
        GraphId::new("hello_world"),
        Constraints::default(),
        log.clone(),
        graph,
        catalog,
        registries,
    );

    // Send an event to trigger execution
    let event = ExternalEvent::mechanical(EventId::new("test_event"), ExternalEventKind::Tick);
    supervisor.on_event(event);

    // Verify the decision log
    let entries = log.entries();
    assert_eq!(entries.len(), 1, "Expected exactly one decision log entry");

    let entry = &entries[0];
    assert_eq!(
        entry.decision,
        Decision::Invoke,
        "Expected Decision::Invoke, got {:?}",
        entry.decision
    );
    assert_eq!(
        entry.termination,
        Some(RunTermination::Completed),
        "Expected RunTermination::Completed, got {:?}",
        entry.termination
    );
    assert_eq!(entry.retry_count, 0, "Expected no retries");
}
