use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NodeOutputRef {
    pub node_id: String,
    pub output_name: String,
}

#[derive(Debug, Clone)]
pub struct SourceNode {
    pub impl_id: String,
    pub parameters: HashMap<String, super::ParameterValue>,
}

#[derive(Debug, Clone)]
pub struct SourceGraph {
    pub nodes: HashMap<String, SourceNode>,
    pub outputs: HashMap<String, NodeOutputRef>,
}
