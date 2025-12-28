use std::collections::HashMap;

use crate::common::{PrimitiveKind, ValidationError};
use crate::compute::{ComputePrimitive, ComputePrimitiveManifest};

pub struct PrimitiveRegistry {
    primitives: HashMap<String, Box<dyn ComputePrimitive>>,
}

impl PrimitiveRegistry {
    pub fn new() -> Self {
        Self {
            primitives: HashMap::new(),
        }
    }

    pub fn validate_manifest(manifest: &ComputePrimitiveManifest) -> Result<(), ValidationError> {
        if manifest.kind != PrimitiveKind::Compute {
            return Err(ValidationError::WrongKind {
                expected: PrimitiveKind::Compute,
                got: manifest.kind.clone(),
            });
        }

        if manifest.side_effects {
            return Err(ValidationError::SideEffectsNotAllowed);
        }

        if !manifest.execution.deterministic {
            return Err(ValidationError::NonDeterministicExecution);
        }

        Ok(())
    }

    pub fn register(&mut self, primitive: Box<dyn ComputePrimitive>) -> Result<(), ValidationError> {
        let manifest = primitive.manifest();

        Self::validate_manifest(manifest)?;

        if self.primitives.contains_key(&manifest.id) {
            return Err(ValidationError::DuplicateId(manifest.id.clone()));
        }

        self.primitives.insert(manifest.id.clone(), primitive);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn ComputePrimitive>> {
        self.primitives.get(id)
    }
}

impl Default for PrimitiveRegistry {
    fn default() -> Self {
        Self::new()
    }
}
