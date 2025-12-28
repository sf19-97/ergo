use std::collections::HashMap;

use crate::action;
use crate::catalog::{build_core_catalog, core_registries};
use crate::cluster::{
    ExpandedEndpoint, ExpandedGraph, ExpandedNode, InputMetadata, OutputMetadata, PrimitiveCatalog,
    PrimitiveKind, PrimitiveMetadata, ValueType,
};
use crate::compute::implementations::{Add, ConstNumber};
use crate::compute::PrimitiveRegistry as ComputeRegistry;
use crate::runtime::run;
use crate::runtime::types::{ExecutionContext, Registries, RuntimeValue};
use crate::source::{SourceKind, SourcePrimitive, SourcePrimitiveManifest, SourceRegistry};
use crate::trigger::TriggerRegistry;

#[derive(Default)]
struct TestCatalog {
    metadata: HashMap<(String, String), PrimitiveMetadata>,
}

impl PrimitiveCatalog for TestCatalog {
    fn get(&self, id: &str, version: &String) -> Option<PrimitiveMetadata> {
        self.metadata
            .get(&(id.to_string(), version.clone()))
            .cloned()
    }
}

fn add_metadata() -> PrimitiveMetadata {
    let mut outputs = HashMap::new();
    outputs.insert(
        "result".to_string(),
        OutputMetadata {
            value_type: ValueType::Number,
            cardinality: crate::cluster::Cardinality::Single,
        },
    );

    PrimitiveMetadata {
        kind: PrimitiveKind::Compute,
        inputs: vec![
            InputMetadata {
                name: "a".to_string(),
                value_type: ValueType::Number,
                required: true,
            },
            InputMetadata {
                name: "b".to_string(),
                value_type: ValueType::Number,
                required: true,
            },
        ],
        outputs,
    }
}

fn source_metadata() -> PrimitiveMetadata {
    let mut outputs = HashMap::new();
    outputs.insert(
        "out".to_string(),
        OutputMetadata {
            value_type: ValueType::Number,
            cardinality: crate::cluster::Cardinality::Single,
        },
    );

    PrimitiveMetadata {
        kind: PrimitiveKind::Source,
        inputs: Vec::new(),
        outputs,
    }
}

#[derive(Clone)]
struct ConstSource {
    manifest: SourcePrimitiveManifest,
    value: f64,
}

impl ConstSource {
    fn new(id: &str, value: f64) -> Self {
        Self {
            manifest: SourcePrimitiveManifest {
                id: id.to_string(),
                version: "v1".to_string(),
                kind: SourceKind::Source,
                inputs: vec![],
                outputs: vec![crate::source::OutputSpec {
                    name: "out".to_string(),
                    value_type: crate::common::ValueType::Number,
                }],
                parameters: vec![],
                execution: crate::source::ExecutionSpec {
                    deterministic: true,
                    cadence: crate::source::Cadence::Continuous,
                },
                state: crate::source::StateSpec { allowed: false },
                side_effects: false,
            },
            value,
        }
    }
}

impl SourcePrimitive for ConstSource {
    fn manifest(&self) -> &SourcePrimitiveManifest {
        &self.manifest
    }

    fn produce(
        &self,
        _parameters: &HashMap<String, crate::source::ParameterValue>,
    ) -> HashMap<String, crate::common::Value> {
        HashMap::from([("out".to_string(), crate::common::Value::Number(self.value))])
    }
}

#[test]
fn unified_runtime_executes_compute_graph() {
    let mut nodes = HashMap::new();
    nodes.insert(
        "src1".to_string(),
        ExpandedNode {
            runtime_id: "src1".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "const1".to_string(),
                version: "v1".to_string(),
            },
            parameters: HashMap::new(),
        },
    );
    nodes.insert(
        "src2".to_string(),
        ExpandedNode {
            runtime_id: "src2".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "const2".to_string(),
                version: "v1".to_string(),
            },
            parameters: HashMap::new(),
        },
    );
    nodes.insert(
        "add1".to_string(),
        ExpandedNode {
            runtime_id: "add1".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "add".to_string(),
                version: "v1".to_string(),
            },
            parameters: HashMap::new(),
        },
    );

    let edges = vec![
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src1".to_string(),
                port_name: "out".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "add1".to_string(),
                port_name: "a".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src2".to_string(),
                port_name: "out".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "add1".to_string(),
                port_name: "b".to_string(),
            },
        },
    ];

    let expanded = ExpandedGraph {
        nodes,
        edges,
        boundary_inputs: Vec::new(),
        boundary_outputs: vec![crate::cluster::OutputPortSpec {
            name: "sum".to_string(),
            maps_to: crate::cluster::OutputRef {
                node_id: "add1".to_string(),
                port_name: "result".to_string(),
            },
        }],
    };

    let mut catalog = TestCatalog::default();
    catalog
        .metadata
        .insert(("add".to_string(), "v1".to_string()), add_metadata());
    catalog
        .metadata
        .insert(("const1".to_string(), "v1".to_string()), source_metadata());
    catalog
        .metadata
        .insert(("const2".to_string(), "v1".to_string()), source_metadata());

    let mut compute_registry = ComputeRegistry::new();
    compute_registry.register(Box::new(Add::new())).unwrap();

    let mut source_registry = SourceRegistry::new();
    source_registry
        .register(Box::new(ConstSource::new("const1", 3.0)))
        .unwrap();
    source_registry
        .register(Box::new(ConstSource::new("const2", 4.0)))
        .unwrap();

    let registries = Registries {
        sources: &source_registry,
        computes: &compute_registry,
        triggers: &TriggerRegistry::new(),
        actions: &crate::action::ActionRegistry::new(),
    };

    let ctx = ExecutionContext {
        trigger_state: HashMap::new(),
    };

    let report = run(&expanded, &catalog, &registries, &ctx).unwrap();
    assert_eq!(report.outputs.get("sum"), Some(&RuntimeValue::Number(7.0)));
}

#[test]
fn parameters_flow_into_compute_execution() {
    let mut nodes = HashMap::new();
    nodes.insert(
        "const_number".to_string(),
        ExpandedNode {
            runtime_id: "const_number".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "const_number".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "value".to_string(),
                crate::cluster::ParameterValue::Number(4.5),
            )]),
        },
    );

    let expanded = ExpandedGraph {
        nodes,
        edges: Vec::new(),
        boundary_inputs: Vec::new(),
        boundary_outputs: vec![crate::cluster::OutputPortSpec {
            name: "out".to_string(),
            maps_to: crate::cluster::OutputRef {
                node_id: "const_number".to_string(),
                port_name: "value".to_string(),
            },
        }],
    };

    let mut catalog = TestCatalog::default();
    catalog.metadata.insert(
        ("const_number".to_string(), "0.1.0".to_string()),
        PrimitiveMetadata {
            kind: PrimitiveKind::Compute,
            inputs: vec![],
            outputs: HashMap::from([(
                "value".to_string(),
                OutputMetadata {
                    value_type: ValueType::Number,
                    cardinality: crate::cluster::Cardinality::Single,
                },
            )]),
        },
    );

    let mut compute_registry = ComputeRegistry::new();
    compute_registry
        .register(Box::new(ConstNumber::new()))
        .unwrap();

    let registries = Registries {
        sources: &SourceRegistry::new(),
        computes: &compute_registry,
        triggers: &TriggerRegistry::new(),
        actions: &action::ActionRegistry::new(),
    };

    let ctx = ExecutionContext {
        trigger_state: HashMap::new(),
    };

    let report = run(&expanded, &catalog, &registries, &ctx).unwrap();
    assert_eq!(report.outputs.get("out"), Some(&RuntimeValue::Number(4.5)));
}

#[test]
fn hello_world_graph_executes_with_core_catalog_and_registries() {
    let mut nodes = HashMap::new();
    nodes.insert(
        "src_a".to_string(),
        ExpandedNode {
            runtime_id: "src_a".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "value".to_string(),
                crate::cluster::ParameterValue::Number(3.0),
            )]),
        },
    );
    nodes.insert(
        "src_b".to_string(),
        ExpandedNode {
            runtime_id: "src_b".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "value".to_string(),
                crate::cluster::ParameterValue::Number(1.0),
            )]),
        },
    );
    nodes.insert(
        "gt1".to_string(),
        ExpandedNode {
            runtime_id: "gt1".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
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
            implementation: crate::cluster::ImplementationInstance {
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
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "ack_action".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "accept".to_string(),
                crate::cluster::ParameterValue::Bool(true),
            )]),
        },
    );

    let edges = vec![
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src_a".to_string(),
                port_name: "value".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "a".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src_b".to_string(),
                port_name: "value".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "b".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "result".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "emit".to_string(),
                port_name: "input".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
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

    let expanded = ExpandedGraph {
        nodes,
        edges,
        boundary_inputs: Vec::new(),
        boundary_outputs: vec![crate::cluster::OutputPortSpec {
            name: "action_outcome".to_string(),
            maps_to: crate::cluster::OutputRef {
                node_id: "act".to_string(),
                port_name: "outcome".to_string(),
            },
        }],
    };

    let catalog = build_core_catalog();
    let registries = core_registries().unwrap();
    let registries = Registries {
        sources: &registries.sources,
        computes: &registries.computes,
        triggers: &registries.triggers,
        actions: &registries.actions,
    };

    let ctx = ExecutionContext {
        trigger_state: HashMap::new(),
    };

    let report = run(&expanded, &catalog, &registries, &ctx).unwrap();
    assert_eq!(
        report.outputs.get("action_outcome"),
        Some(&RuntimeValue::Event(
            crate::runtime::types::RuntimeEvent::Action(crate::action::ActionOutcome::Filled)
        ))
    );
}

#[test]
fn validation_fails_on_missing_required_input() {
    // Same graph as hello_world but with edge src_a -> gt1 removed
    // This should cause validation to fail: gt1 is missing required input "a"
    let mut nodes = HashMap::new();
    nodes.insert(
        "src_a".to_string(),
        ExpandedNode {
            runtime_id: "src_a".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "value".to_string(),
                crate::cluster::ParameterValue::Number(3.0),
            )]),
        },
    );
    nodes.insert(
        "src_b".to_string(),
        ExpandedNode {
            runtime_id: "src_b".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "number_source".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "value".to_string(),
                crate::cluster::ParameterValue::Number(1.0),
            )]),
        },
    );
    nodes.insert(
        "gt1".to_string(),
        ExpandedNode {
            runtime_id: "gt1".to_string(),
            authoring_path: vec![],
            implementation: crate::cluster::ImplementationInstance {
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
            implementation: crate::cluster::ImplementationInstance {
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
            implementation: crate::cluster::ImplementationInstance {
                impl_id: "ack_action".to_string(),
                version: "0.1.0".to_string(),
            },
            parameters: HashMap::from([(
                "accept".to_string(),
                crate::cluster::ParameterValue::Bool(true),
            )]),
        },
    );

    // Missing edge: src_a -> gt1:a (first edge removed)
    let edges = vec![
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "src_b".to_string(),
                port_name: "value".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "b".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
            from: ExpandedEndpoint::NodePort {
                node_id: "gt1".to_string(),
                port_name: "result".to_string(),
            },
            to: ExpandedEndpoint::NodePort {
                node_id: "emit".to_string(),
                port_name: "input".to_string(),
            },
        },
        crate::cluster::ExpandedEdge {
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

    let expanded = ExpandedGraph {
        nodes,
        edges,
        boundary_inputs: Vec::new(),
        boundary_outputs: vec![crate::cluster::OutputPortSpec {
            name: "action_outcome".to_string(),
            maps_to: crate::cluster::OutputRef {
                node_id: "act".to_string(),
                port_name: "outcome".to_string(),
            },
        }],
    };

    let catalog = build_core_catalog();

    // Validation should fail with MissingRequiredInput
    let result = crate::runtime::validate::validate(&expanded, &catalog);
    assert!(result.is_err(), "Expected validation to fail");
    match result.unwrap_err() {
        crate::runtime::types::ValidationError::MissingRequiredInput { node, input } => {
            assert_eq!(node, "gt1");
            assert_eq!(input, "a");
        }
        other => panic!("Expected MissingRequiredInput, got {:?}", other),
    }
}
