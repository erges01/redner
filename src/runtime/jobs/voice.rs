use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};

use crate::runtime::executor::runner::{NodeRunner, RunnerFuture};
use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::providers::voice::{VoiceProvider, VoiceGenerationParams};

pub struct VoiceRunner {
    pub provider: Arc<dyn VoiceProvider>,
}

impl VoiceRunner {
    pub fn new(provider: Arc<dyn VoiceProvider>) -> Self {
        Self { provider }
    }
}

impl NodeRunner for VoiceRunner {
    fn handles(&self) -> NodeType {
        NodeType::Voice
    }

    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        context: &'a ExecutionContext,
    ) -> RunnerFuture<'a> {
        let node_id = node.id.clone();
        
        // 1. Extract inputs from the graph node
        // Fallback to dummy data if inputs aren't perfectly wired yet
        let text = node.inputs.get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("This is a default test generation for Redner.")
            .to_string();
            
        let voice_id = node.config.get("voice_id")
            .and_then(|v| v.as_str())
            .unwrap_or("default_voice")
            .to_string();

        let provider = self.provider.clone();
        
        // 2. Setup the output file path in our workspace
        let output_filename = format!("{}.mp3", node_id);
        let output_path = context.workspace_dir.join(&output_filename);

        Box::pin(async move {
            println!("   🎙️ [VoiceRunner] Generating audio for node {}...", node_id);
            
            // Ensure the workspace directory exists
            if let Err(e) = tokio::fs::create_dir_all(&context.workspace_dir).await {
                return Err(format!("Failed to create workspace directory: {}", e));
            }

            let params = VoiceGenerationParams {
                text,
                voice_id,
                speed: 1.0,
            };

            // 3. Call the actual AI provider!
            match provider.generate_audio(params, output_path.clone()).await {
                Ok(_) => {
                    let mut outputs = HashMap::new();
                    outputs.insert("status".to_string(), json!("success"));
                    outputs.insert("audio_file_path".to_string(), json!(output_path.to_string_lossy()));
                    
                    println!("   ✅ [VoiceRunner] Audio saved to {:?}", output_path);
                    Ok(outputs)
                },
                Err(e) => Err(format!("Voice generation failed: {}", e))
            }
        })
    }
}