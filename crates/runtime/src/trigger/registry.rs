use std::collections::HashMap;

use super::{
    OutputSpec, TriggerKind, TriggerPrimitive, TriggerPrimitiveManifest, TriggerValidationError,
    TriggerValueType,
};

pub struct TriggerRegistry {
    primitives: HashMap<String, Box<dyn TriggerPrimitive>>,
}

impl TriggerRegistry {
    pub fn new() -> Self {
        Self {
            primitives: HashMap::new(),
        }
    }

    pub fn validate_manifest(
        manifest: &TriggerPrimitiveManifest,
    ) -> Result<(), TriggerValidationError> {
        if manifest.kind != TriggerKind::Trigger {
            return Err(TriggerValidationError::WrongKind {
                expected: TriggerKind::Trigger,
                got: manifest.kind.clone(),
            });
        }

        if manifest.side_effects {
            return Err(TriggerValidationError::SideEffectsNotAllowed);
        }

        if !manifest.execution.deterministic {
            return Err(TriggerValidationError::NonDeterministicExecution);
        }

        // TRG-STATE-1: Triggers must be stateless.
        // Temporal patterns requiring memory must be implemented as clusters.
        if manifest.state.allowed {
            return Err(TriggerValidationError::StatefulTriggerNotAllowed {
                trigger_id: manifest.id.clone(),
            });
        }

        Self::validate_outputs(&manifest.outputs)?;

        Ok(())
    }

    fn validate_outputs(outputs: &[OutputSpec]) -> Result<(), TriggerValidationError> {
        if outputs.len() != 1 {
            return Err(TriggerValidationError::UndeclaredOutput {
                primitive: "trigger".to_string(),
                output: "expected exactly one event output".to_string(),
            });
        }

        let output = &outputs[0];
        if output.value_type != TriggerValueType::Event {
            return Err(TriggerValidationError::InvalidOutputType {
                output: output.name.clone(),
                expected: TriggerValueType::Event,
                got: output.value_type.clone(),
            });
        }

        Ok(())
    }

    pub fn register(
        &mut self,
        primitive: Box<dyn TriggerPrimitive>,
    ) -> Result<(), TriggerValidationError> {
        let manifest = primitive.manifest();

        Self::validate_manifest(manifest)?;

        if self.primitives.contains_key(&manifest.id) {
            return Err(TriggerValidationError::DuplicateId(manifest.id.clone()));
        }

        self.primitives.insert(manifest.id.clone(), primitive);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn TriggerPrimitive>> {
        self.primitives.get(id)
    }
}

impl Default for TriggerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trigger::{Cadence, ExecutionSpec, InputSpec, StateSpec};

    fn make_valid_manifest() -> TriggerPrimitiveManifest {
        TriggerPrimitiveManifest {
            id: "test_trigger".to_string(),
            version: "0.1.0".to_string(),
            kind: TriggerKind::Trigger,
            inputs: vec![InputSpec {
                name: "input".to_string(),
                value_type: TriggerValueType::Bool,
                required: true,
                cardinality: super::super::Cardinality::Single,
            }],
            outputs: vec![OutputSpec {
                name: "event".to_string(),
                value_type: TriggerValueType::Event,
            }],
            parameters: vec![],
            execution: ExecutionSpec {
                deterministic: true,
                cadence: Cadence::Continuous,
            },
            state: StateSpec {
                allowed: false,
                description: None,
            },
            side_effects: false,
        }
    }

    #[test]
    fn trg_state_1_stateful_trigger_rejected() {
        let mut manifest = make_valid_manifest();
        manifest.state.allowed = true;

        let result = TriggerRegistry::validate_manifest(&manifest);

        assert!(matches!(
            result,
            Err(TriggerValidationError::StatefulTriggerNotAllowed { trigger_id })
            if trigger_id == "test_trigger"
        ));
    }

    #[test]
    fn stateless_trigger_accepted() {
        let manifest = make_valid_manifest();

        let result = TriggerRegistry::validate_manifest(&manifest);

        assert!(result.is_ok());
    }
}
