use crate::common::{PrimitiveKind, ValueType};
use crate::compute::{
    Cadence, ComputePrimitiveManifest, ExecutionSpec, OutputSpec, ParameterSpec, StateSpec,
};

pub fn const_number_manifest() -> ComputePrimitiveManifest {
    ComputePrimitiveManifest {
        id: "const_number".to_string(),
        version: "0.1.0".to_string(),
        kind: PrimitiveKind::Compute,
        inputs: vec![],
        outputs: vec![OutputSpec {
            name: "value".to_string(),
            value_type: ValueType::Number,
        }],
        parameters: vec![ParameterSpec {
            name: "value".to_string(),
            value_type: ValueType::Number,
            default: None,
        }],
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
