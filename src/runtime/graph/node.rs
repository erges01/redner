use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Pending,
    Ready,      // Dependencies met, waiting for executor
    Running,
    Completed,
    Failed(String),
    Cached,     // Skipped execution because output already exists
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Narration,
    Voice,
    LipSync,
    AvatarAnimation,
    SceneComposition,
    Caption,
    Render,
    // Future proofing:
    Thumbnail,
    Translation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNode {
    pub id: String,
    pub node_type: NodeType,
    pub status: NodeStatus,
    
    /// The specific configuration for this node (e.g., voice_id, model_name)
    pub config: Value,
    
    /// Nodes that MUST complete before this one starts
    pub dependencies: Vec<String>,
    
    /// Data passed into the node at runtime
    pub inputs: HashMap<String, Value>,
    
    /// Data produced by the node (e.g., S3 URL of generated audio)
    pub outputs: HashMap<String, Value>,
}

impl RuntimeNode {
    pub fn new(id: impl Into<String>, node_type: NodeType, dependencies: Vec<String>) -> Self {
        Self {
            id: id.into(),
            node_type,
            status: NodeStatus::Pending,
            config: Value::Null,
            dependencies,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    /// Checks if this node is ready to be executed
    pub fn is_ready(&self) -> bool {
        self.status == NodeStatus::Ready || self.status == NodeStatus::Pending
    }
}