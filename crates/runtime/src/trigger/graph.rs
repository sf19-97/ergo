use std::collections::HashMap;

use super::ParameterValue;

#[derive(Debug, Clone)]
pub struct NodeOutputRef {
    pub node_id: String,
    pub output_name: String,
}

#[derive(Debug, Clone)]
pub enum InputBinding {
    NodeOutput(NodeOutputRef),
    GraphInput(String),
}

#[derive(Debug, Clone)]
pub struct TriggerNode {
    pub impl_id: String,
    pub input_bindings: HashMap<String, InputBinding>,
    pub parameters: HashMap<String, ParameterValue>,
}

#[derive(Debug, Clone)]
pub struct TriggerGraph {
    pub nodes: HashMap<String, TriggerNode>,
    pub outputs: HashMap<String, NodeOutputRef>,
}
