use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::executor::runner::{NodeRunner, RunnerFuture};
use crate::runtime::providers::scene::SceneProvider;

pub struct SceneRunner {
    provider: Arc<dyn SceneProvider>,
}

impl SceneRunner {
    pub fn new(provider: Arc<dyn SceneProvider>) -> Self {
        Self { provider }
    }
}

// 🛑 Notice: No #[async_trait] here!
impl NodeRunner for SceneRunner {
    fn handles(&self) -> NodeType {
        NodeType::Scene
    }

    // Matches your runner.rs blueprint EXACTLY
    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        _context: &'a ExecutionContext,
    ) -> RunnerFuture<'a> {
        // Wrap the async logic in the Box::pin just like the trait expects
        Box::pin(async move {
            println!("   🎨 [SceneRunner] Processing background scene for node {}...", node.id);

            let prompt = node.inputs.get("prompt")
                .and_then(|v| v.as_str())
                .unwrap_or("Modern dark-mode software engineering background");

            let output_path = PathBuf::from(format!("./temp/{}_scene.json", node.id));

            // Run the mock generation
            self.provider.generate_scene(prompt, output_path.clone()).await?;

            // Return a HashMap<String, Value> exactly as RunnerFuture demands
            let mut results = HashMap::new();
            results.insert(
                "scene_output".to_string(),
                serde_json::json!(output_path.to_string_lossy())
            );
            results.insert(
                "prompt_used".to_string(),
                serde_json::json!(prompt)
            );

            Ok(results)
        })
    }
}