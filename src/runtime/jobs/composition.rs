use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::executor::runner::{NodeRunner, RunnerFuture};
use crate::runtime::providers::composition::CompositionProvider;

pub struct CompositionRunner {
    provider: Arc<dyn CompositionProvider>,
}

impl CompositionRunner {
    pub fn new(provider: Arc<dyn CompositionProvider>) -> Self {
        Self { provider }
    }
}

impl NodeRunner for CompositionRunner {
    fn handles(&self) -> NodeType {
        NodeType::Composition // We need to add this to your enum!
    }

    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        _context: &'a ExecutionContext,
    ) -> RunnerFuture<'a> {
        Box::pin(async move {
            println!("   🎞️ [CompositionRunner] Assembling final timeline for node {}...", node.id);

            // In a real graph, we would fetch these paths from the context's previous node outputs.
            // For this milestone test, we'll extract them from inputs or use fallbacks.
            let scene_path = node.inputs.get("scene_path").and_then(|v| v.as_str()).unwrap_or("./temp/fallback_scene.json");
            let voice_path = node.inputs.get("voice_path").and_then(|v| v.as_str()).unwrap_or("./temp/fallback_voice.mp3");
            let lipsync_path = node.inputs.get("lipsync_path").and_then(|v| v.as_str()).unwrap_or("./temp/fallback_visemes.json");

            let output_path = PathBuf::from(format!("./temp/{}_timeline.json", node.id));

            self.provider.generate_timeline(scene_path, voice_path, lipsync_path, output_path.clone()).await?;

            let mut results = HashMap::new();
            results.insert(
                "timeline_file".to_string(),
                serde_json::json!(output_path.to_string_lossy())
            );

            Ok(results)
        })
    }
}