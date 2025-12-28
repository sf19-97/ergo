use crate::common::{PrimitiveKind, ValueType};
use crate::compute::{
    Cadence, ComputePrimitiveManifest, ExecutionSpec, InputSpec, OutputSpec, StateSpec,
};

pub fn not_manifest() -> ComputePrimitiveManifest {
    ComputePrimitiveManifest {
        id: "not".to_string(),
        version: "0.1.0".to_string(),
        kind: PrimitiveKind::Compute,
        inputs: vec![InputSpec {
            name: "value".to_string(),
            value_type: ValueType::Bool,
            required: true,
        }],
        outputs: vec![OutputSpec {
            name: "result".to_string(),
            value_type: ValueType::Bool,
        }],
        parameters: vec![],
        execution: ExecutionSpec {
            deterministic: true,
            cadence: Cadence::Continuous,
        },
        state: StateSpec {
            stateful: false,
            rolling_window: None,
        },
        side_effects: false,
    }
}
