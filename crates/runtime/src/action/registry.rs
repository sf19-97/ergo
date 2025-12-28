use std::collections::HashMap;

use super::{
    ActionKind, ActionPrimitive, ActionPrimitiveManifest, ActionValidationError, ActionValueType,
    OutputSpec,
};

pub struct ActionRegistry {
    primitives: HashMap<String, Box<dyn ActionPrimitive>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            primitives: HashMap::new(),
        }
    }

    pub fn validate_manifest(
        manifest: &ActionPrimitiveManifest,
    ) -> Result<(), ActionValidationError> {
        if manifest.kind != ActionKind::Action {
            return Err(ActionValidationError::WrongKind {
                expected: ActionKind::Action,
                got: manifest.kind.clone(),
            });
        }

        if !manifest.side_effects {
            return Err(ActionValidationError::SideEffectsRequired);
        }

        if manifest.execution.retryable {
            return Err(ActionValidationError::RetryNotAllowed);
        }

        if !manifest.execution.deterministic {
            return Err(ActionValidationError::NonDeterministicExecution);
        }

        if manifest.state.allowed {
            return Err(ActionValidationError::StateNotAllowed);
        }

        if !manifest
            .inputs
            .iter()
            .any(|input| input.value_type == ActionValueType::Event)
        {
            return Err(ActionValidationError::EventInputRequired);
        }

        Self::validate_outputs(&manifest.outputs)?;

        Ok(())
    }

    fn validate_outputs(outputs: &[OutputSpec]) -> Result<(), ActionValidationError> {
        if outputs.len() != 1 {
            return Err(ActionValidationError::UndeclaredOutput {
                primitive: "action".to_string(),
                output: "expected exactly one outcome event".to_string(),
            });
        }

        let output = &outputs[0];
        if output.name != "outcome" || output.value_type != ActionValueType::Event {
            return Err(ActionValidationError::InvalidOutputType {
                output: output.name.clone(),
                expected: ActionValueType::Event,
                got: output.value_type.clone(),
            });
        }

        Ok(())
    }

    pub fn register(
        &mut self,
        primitive: Box<dyn ActionPrimitive>,
    ) -> Result<(), ActionValidationError> {
        let manifest = primitive.manifest();

        Self::validate_manifest(manifest)?;

        if self.primitives.contains_key(&manifest.id) {
            return Err(ActionValidationError::DuplicateId(manifest.id.clone()));
        }

        self.primitives.insert(manifest.id.clone(), primitive);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn ActionPrimitive>> {
        self.primitives.get(id)
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
