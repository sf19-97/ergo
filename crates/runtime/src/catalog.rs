use std::collections::HashMap;

use crate::action::{
    implementations::{ack_action_manifest, annotate_action_manifest},
    AckAction, AnnotateAction, ActionRegistry, ActionValidationError, ActionValueType,
};
use crate::cluster::{
    Cardinality, InputMetadata, OutputMetadata, PrimitiveCatalog, PrimitiveKind, PrimitiveMetadata, ValueType,
    Version,
};
use crate::common;
use crate::common::ValidationError;
use crate::compute::implementations::{
    add::add_manifest, and::and_manifest, const_bool::const_bool_manifest, const_number::const_number_manifest,
    divide::divide_manifest, eq::eq_manifest, gt::gt_manifest, lt::lt_manifest, multiply::multiply_manifest,
    negate::negate_manifest, neq::neq_manifest, not::not_manifest, or::or_manifest, select::select_manifest,
    subtract::subtract_manifest, Add, And, ConstBool, ConstNumber, Divide, Eq, Gt, Lt, Multiply, Negate, Neq, Not,
    Or, Select, Subtract,
};
use crate::compute::{ComputePrimitiveManifest, PrimitiveRegistry as ComputeRegistry};
use crate::source::{
    implementations::{boolean_source_manifest, number_source_manifest},
    BooleanSource, NumberSource, SourceRegistry, SourceValidationError,
};
use crate::trigger::{
    implementations::emit_if_true::emit_if_true_manifest, EmitIfTrue, TriggerRegistry, TriggerValidationError,
    TriggerValueType,
};

#[derive(Debug)]
pub enum CoreRegistrationError {
    Source(SourceValidationError),
    Compute(ValidationError),
    Trigger(TriggerValidationError),
    Action(ActionValidationError),
}

pub struct CoreRegistries {
    pub sources: SourceRegistry,
    pub computes: ComputeRegistry,
    pub triggers: TriggerRegistry,
    pub actions: ActionRegistry,
}

impl CoreRegistries {
    pub fn new(
        sources: SourceRegistry,
        computes: ComputeRegistry,
        triggers: TriggerRegistry,
        actions: ActionRegistry,
    ) -> Self {
        Self { sources, computes, triggers, actions }
    }
}

pub fn core_registries() -> Result<CoreRegistries, CoreRegistrationError> {
    let mut sources = SourceRegistry::new();
    sources.register(Box::new(NumberSource::new())).map_err(CoreRegistrationError::Source)?;
    sources.register(Box::new(BooleanSource::new())).map_err(CoreRegistrationError::Source)?;

    let mut computes = ComputeRegistry::new();
    computes.register(Box::new(ConstNumber::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(ConstBool::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Add::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Subtract::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Multiply::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Divide::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Negate::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Gt::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Lt::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Eq::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Neq::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(And::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Or::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Not::new())).map_err(CoreRegistrationError::Compute)?;
    computes.register(Box::new(Select::new())).map_err(CoreRegistrationError::Compute)?;

    let mut triggers = TriggerRegistry::new();
    triggers.register(Box::new(EmitIfTrue::new())).map_err(CoreRegistrationError::Trigger)?;

    let mut actions = ActionRegistry::new();
    actions.register(Box::new(AckAction::new())).map_err(CoreRegistrationError::Action)?;
    actions.register(Box::new(AnnotateAction::new())).map_err(CoreRegistrationError::Action)?;

    Ok(CoreRegistries::new(sources, computes, triggers, actions))
}

pub struct CorePrimitiveCatalog {
    metadata: HashMap<(String, Version), PrimitiveMetadata>,
}

impl CorePrimitiveCatalog {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    pub fn register_compute(&mut self, manifest: ComputePrimitiveManifest) {
        let inputs = manifest
            .inputs
            .into_iter()
            .map(|i| InputMetadata {
                name: i.name,
                value_type: map_common_value_type(i.value_type),
                required: i.required,
            })
            .collect();

        let outputs = manifest
            .outputs
            .into_iter()
            .map(|o| {
                (
                    o.name,
                    OutputMetadata {
                        value_type: map_common_value_type(o.value_type),
                        cardinality: Cardinality::Single,
                    },
                )
            })
            .collect();

        self.metadata.insert(
            (manifest.id.clone(), manifest.version.clone()),
            PrimitiveMetadata {
                kind: PrimitiveKind::Compute,
                inputs,
                outputs,
            },
        );
    }

    pub fn register_trigger(&mut self, manifest: crate::trigger::TriggerPrimitiveManifest) {
        let inputs = manifest
            .inputs
            .into_iter()
            .map(|i| InputMetadata {
                name: i.name,
                value_type: map_trigger_value_type(i.value_type),
                required: i.required,
            })
            .collect();

        let outputs = manifest
            .outputs
            .into_iter()
            .map(|o| {
                (
                    o.name,
                    OutputMetadata {
                        value_type: map_trigger_value_type(o.value_type),
                        cardinality: Cardinality::Single,
                    },
                )
            })
            .collect();

        self.metadata.insert(
            (manifest.id.clone(), manifest.version.clone()),
            PrimitiveMetadata {
                kind: PrimitiveKind::Trigger,
                inputs,
                outputs,
            },
        );
    }

    pub fn register_source(&mut self, manifest: crate::source::SourcePrimitiveManifest) {
        let inputs = vec![];
        let outputs = manifest
            .outputs
            .into_iter()
            .map(|o| {
                (
                    o.name,
                    OutputMetadata {
                        value_type: map_common_value_type(o.value_type),
                        cardinality: Cardinality::Single,
                    },
                )
            })
            .collect();

        self.metadata.insert(
            (manifest.id.clone(), manifest.version.clone()),
            PrimitiveMetadata {
                kind: PrimitiveKind::Source,
                inputs,
                outputs,
            },
        );
    }

    pub fn register_action(&mut self, manifest: crate::action::ActionPrimitiveManifest) {
        let inputs = manifest
            .inputs
            .into_iter()
            .map(|i| InputMetadata {
                name: i.name,
                value_type: map_action_value_type(i.value_type),
                required: i.required,
            })
            .collect();

        let outputs = manifest
            .outputs
            .into_iter()
            .map(|o| {
                (
                    o.name,
                    OutputMetadata {
                        value_type: map_action_value_type(o.value_type),
                        cardinality: Cardinality::Single,
                    },
                )
            })
            .collect();

        self.metadata.insert(
            (manifest.id.clone(), manifest.version.clone()),
            PrimitiveMetadata {
                kind: PrimitiveKind::Action,
                inputs,
                outputs,
            },
        );
    }
}

impl PrimitiveCatalog for CorePrimitiveCatalog {
    fn get(&self, id: &str, version: &Version) -> Option<PrimitiveMetadata> {
        self.metadata.get(&(id.to_string(), version.clone())).cloned()
    }
}

pub fn build_core_catalog() -> CorePrimitiveCatalog {
    let mut catalog = CorePrimitiveCatalog::new();

    // Sources
    catalog.register_source(number_source_manifest());
    catalog.register_source(boolean_source_manifest());

    // Computes
    catalog.register_compute(const_number_manifest());
    catalog.register_compute(const_bool_manifest());
    catalog.register_compute(add_manifest());
    catalog.register_compute(subtract_manifest());
    catalog.register_compute(multiply_manifest());
    catalog.register_compute(divide_manifest());
    catalog.register_compute(negate_manifest());
    catalog.register_compute(gt_manifest());
    catalog.register_compute(lt_manifest());
    catalog.register_compute(eq_manifest());
    catalog.register_compute(neq_manifest());
    catalog.register_compute(and_manifest());
    catalog.register_compute(or_manifest());
    catalog.register_compute(not_manifest());
    catalog.register_compute(select_manifest());

    // Triggers
    catalog.register_trigger(emit_if_true_manifest());

    // Actions
    catalog.register_action(ack_action_manifest());
    catalog.register_action(annotate_action_manifest());

    catalog
}

fn map_common_value_type(value_type: common::ValueType) -> ValueType {
    match value_type {
        common::ValueType::Number => ValueType::Number,
        common::ValueType::Series => ValueType::Series,
        common::ValueType::Bool => ValueType::Bool,
    }
}

fn map_trigger_value_type(value_type: TriggerValueType) -> ValueType {
    match value_type {
        TriggerValueType::Number => ValueType::Number,
        TriggerValueType::Series => ValueType::Series,
        TriggerValueType::Bool => ValueType::Bool,
        TriggerValueType::Event => ValueType::Event,
    }
}

fn map_action_value_type(value_type: ActionValueType) -> ValueType {
    match value_type {
        ActionValueType::Event => ValueType::Event,
        ActionValueType::Number => ValueType::Number,
        ActionValueType::Bool => ValueType::Bool,
        ActionValueType::String => ValueType::String,
    }
}
