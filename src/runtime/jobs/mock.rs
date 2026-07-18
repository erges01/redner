use std::collections::HashMap;
use serde_json::{json, Value};
use crate::runtime::executor::runner::{NodeRunner, RunnerFuture};
use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::context::execution_context::ExecutionContext;

// 👇 Make sure this is here and marked as `pub`!
pub struct MockRunner {
    pub node_type: NodeType,
}

impl NodeRunner for MockRunner {
    fn handles(&self) -> NodeType {
        self.node_type.clone()
    }

    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        _context: &'a ExecutionContext,
    ) -> RunnerFuture<'a> {
        let id = node.id.clone();
        let n_type = self.node_type.clone();
        
        Box::pin(async move {
            println!("   ⚙️ [MockRunner] Processing {} ({:?})...", id, n_type);
            
            // Simulate AI generation time (1 second)
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            // Return fake outputs
            let mut outputs = HashMap::new();
            outputs.insert("status".to_string(), json!("success"));
            outputs.insert("file_path".to_string(), json!(format!("/temp_workspace/{}.mp4", id)));
            
            Ok(outputs)
        })
    }
}