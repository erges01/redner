use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::executor::runner::{NodeRunner, RunnerFuture};
use crate::runtime::providers::render::RenderProvider;

pub struct RenderRunner {
    provider: Arc<dyn RenderProvider>,
}

impl RenderRunner {
    pub fn new(provider: Arc<dyn RenderProvider>) -> Self {
        Self { provider }
    }
}

impl NodeRunner for RenderRunner {
    fn handles(&self) -> NodeType {
        NodeType::Render
    }

    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        _context: &'a ExecutionContext,
    ) -> RunnerFuture<'a> {
        Box::pin(async move {
            println!("   🎬 [RenderRunner] Crunching timeline into final MP4 for node {}...", node.id);

            let timeline_path = node.inputs.get("timeline_path")
                .and_then(|v| v.as_str())
                .unwrap_or("./temp/step_5_composition_timeline.json");

            let output_path = PathBuf::from(format!("./temp/{}_final_export.mp4", node.id));

            self.provider.render_timeline(timeline_path, output_path.clone()).await?;

            let mut results = HashMap::new();
            results.insert(
                "video_export".to_string(),
                serde_json::json!(output_path.to_string_lossy())
            );

            Ok(results)
        })
    }
}