use std::collections::HashMap;
use std::sync::Arc;
use crate::runtime::graph::node::NodeType;
use crate::runtime::executor::runner::NodeRunner;

#[derive(Clone)]
pub struct RunnerRegistry {
    // We map the string representation of NodeType to the actual Runner implementation
    runners: HashMap<String, Arc<dyn NodeRunner>>,
}

impl RunnerRegistry {
    pub fn new() -> Self {
        Self {
            runners: HashMap::new(),
        }
    }

    /// Register a new capability into the Redner Runtime
    pub fn register(&mut self, runner: impl NodeRunner + 'static) {
        // Convert the enum to its snake_case string representation
        let type_str = serde_json::to_string(&runner.handles())
            .unwrap_or_default()
            .replace("\"", ""); // strip quotes
            
        self.runners.insert(type_str, Arc::new(runner));
    }

    /// Retrieve a runner for a specific node type
    pub fn get(&self, node_type: &NodeType) -> Option<Arc<dyn NodeRunner>> {
        let type_str = serde_json::to_string(node_type)
            .unwrap_or_default()
            .replace("\"", "");
            
        self.runners.get(&type_str).cloned()
    }
}