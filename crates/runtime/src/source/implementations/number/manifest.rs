use crate::common::ValueType;
use crate::source::{
    Cadence, ExecutionSpec, OutputSpec, ParameterSpec, ParameterValue, SourceKind,
    SourcePrimitiveManifest, StateSpec,
};

pub fn number_source_manifest() -> SourcePrimitiveManifest {
    SourcePrimitiveManifest {
        id: "number_source".to_string(),
        version: "0.1.0".to_string(),
        kind: SourceKind::Source,
        inputs: vec![],
        outputs: vec![OutputSpec {
            name: "value".to_string(),
            value_type: ValueType::Number,
        }],
        parameters: vec![ParameterSpec {
            name: "value".to_string(),
            value_type: ParameterValue::Number(0.0).value_type(),
            default: Some(ParameterValue::Number(0.0)),
            bounds: None,
        }],
        execution: ExecutionSpec {
            deterministic: true,
            cadence: Cadence::Continuous,
        },
        state: StateSpec { allowed: false },
        side_effects: false,
    }
}
