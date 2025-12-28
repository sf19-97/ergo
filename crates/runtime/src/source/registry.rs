use std::collections::HashMap;

use super::{Cadence, SourceKind, SourcePrimitive, SourcePrimitiveManifest, SourceValidationError};

pub struct SourceRegistry {
    primitives: HashMap<String, Box<dyn SourcePrimitive>>,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self {
            primitives: HashMap::new(),
        }
    }

    pub fn validate_manifest(
        manifest: &SourcePrimitiveManifest,
    ) -> Result<(), SourceValidationError> {
        if manifest.kind != SourceKind::Source {
            return Err(SourceValidationError::WrongKind {
                expected: SourceKind::Source,
                got: manifest.kind.clone(),
            });
        }

        if !manifest.inputs.is_empty() {
            return Err(SourceValidationError::InputsNotAllowed);
        }

        if manifest.side_effects {
            return Err(SourceValidationError::SideEffectsNotAllowed);
        }

        if !manifest.execution.deterministic {
            return Err(SourceValidationError::NonDeterministicExecution);
        }

        if manifest.execution.cadence != Cadence::Continuous {
            return Err(SourceValidationError::InvalidCadence);
        }

        if manifest.state.allowed {
            return Err(SourceValidationError::StateNotAllowed);
        }

        if manifest.outputs.is_empty() {
            return Err(SourceValidationError::OutputsRequired);
        }

        Ok(())
    }

    pub fn register(
        &mut self,
        primitive: Box<dyn SourcePrimitive>,
    ) -> Result<(), SourceValidationError> {
        let manifest = primitive.manifest();

        Self::validate_manifest(manifest)?;

        if self.primitives.contains_key(&manifest.id) {
            return Err(SourceValidationError::DuplicateId(manifest.id.clone()));
        }

        self.primitives.insert(manifest.id.clone(), primitive);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn SourcePrimitive>> {
        self.primitives.get(id)
    }
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
