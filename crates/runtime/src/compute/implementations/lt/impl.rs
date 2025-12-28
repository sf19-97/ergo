use std::collections::HashMap;

use crate::common::Value;
use crate::compute::{ComputePrimitive, ComputePrimitiveManifest, PrimitiveState};

use super::manifest::lt_manifest;

pub struct Lt {
    manifest: ComputePrimitiveManifest,
}

impl Lt {
    pub fn new() -> Self {
        Self {
            manifest: lt_manifest(),
        }
    }
}

impl Default for Lt {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePrimitive for Lt {
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

        HashMap::from([("result".to_string(), Value::Bool(a < b))])
    }
}
