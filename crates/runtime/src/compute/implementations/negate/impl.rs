use std::collections::HashMap;

use crate::common::Value;
use crate::compute::{ComputePrimitive, ComputePrimitiveManifest, PrimitiveState};

use super::manifest::negate_manifest;

pub struct Negate {
    manifest: ComputePrimitiveManifest,
}

impl Negate {
    pub fn new() -> Self {
        Self {
            manifest: negate_manifest(),
        }
    }
}

impl Default for Negate {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePrimitive for Negate {
    fn manifest(&self) -> &ComputePrimitiveManifest {
        &self.manifest
    }

    fn compute(
        &self,
        inputs: &HashMap<String, Value>,
        _parameters: &HashMap<String, Value>,
        _state: Option<&mut PrimitiveState>,
    ) -> HashMap<String, Value> {
        let value = inputs
            .get("value")
            .and_then(|v| v.as_number())
            .expect("missing required numeric input 'value'");

        HashMap::from([("result".to_string(), Value::Number(-value))])
    }
}
