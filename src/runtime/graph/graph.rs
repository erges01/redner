use super::node::{RuntimeNode, NodeStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeGraph {
    pub id: String,
    pub nodes: HashMap<String, RuntimeNode>,
}

impl RuntimeGraph {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: RuntimeNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Returns a list of node IDs that have no pending dependencies
    /// and are ready to be picked up by the Executor.
    pub fn get_runnable_nodes(&self) -> Vec<String> {
        self.nodes
            .values()
            .filter(|node| {
                // Node must be Pending or Ready
                if !node.is_ready() {
                    return false;
                }
                
                // All dependencies must be Completed or Cached
                node.dependencies.iter().all(|dep_id| {
                    if let Some(dep_node) = self.nodes.get(dep_id) {
                        matches!(dep_node.status, NodeStatus::Completed | NodeStatus::Cached)
                    } else {
                        false // Dependency doesn't exist (malformed graph)
                    }
                })
            })
            .map(|node| node.id.clone())
            .collect()
    }

    /// Updates a node's status
    pub fn update_node_status(&mut self, node_id: &str, status: NodeStatus) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.status = status;
        }
    }
}