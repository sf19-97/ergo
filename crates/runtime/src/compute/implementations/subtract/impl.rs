use std::collections::HashMap;

use crate::common::Value;
use crate::compute::{ComputePrimitive, ComputePrimitiveManifest, PrimitiveState};

use super::manifest::subtract_manifest;

pub struct Subtract {
    manifest: ComputePrimitiveManifest,
}

impl Subtract {
    pub fn new() -> Self {
        Self {
            manifest: subtract_manifest(),
        }
    }
}

impl Default for Subtract {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePrimitive for Subtract {
    fn manifest(&self) -> &ComputePrimitiveManifest {
        &self.manifest
    }

    fn compute(
        &self,
        inputs: &HashMap<String, Value>,
        _parameters: &HashMap<String, Value>,
        _state: Option<&mut PrimitiveState>,
    ) -> HashMap<String, Value> {
        let a = inputs
            .get("a")
            .and_then(|v| v.as_number())
            .expect("missing required numeric input 'a'");
        let b = inputs
            .get("b")
            .and_then(|v| v.as_number())
            .expect("missing required numeric input 'b'");

        HashMap::from([("result".to_string(), Value::Number(a - b))])
    }
}
