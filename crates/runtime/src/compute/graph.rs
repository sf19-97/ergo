use std::collections::HashMap;

use crate::common::Value;

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
pub struct GraphNode {
    pub impl_id: String,
    pub input_bindings: HashMap<String, InputBinding>,
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ComputeGraph {
    pub nodes: HashMap<String, GraphNode>,
    pub outputs: HashMap<String, NodeOutputRef>,
}
