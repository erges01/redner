use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::platform::api::TimelineApi;

/// 1. THE UNIVERSAL TOOL CONTRACT
/// Any feature in Redner must implement this to be usable by Plugins or AI.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    
    /// Returns an OpenAI/Claude compatible JSON Schema 
    /// so an LLM knows exactly how to use this tool!
    fn schema(&self) -> Value;
    
    /// The actual execution logic
    async fn execute(&self, args: Value) -> Result<Value, String>;
}

/// 2. THE TOOL REGISTRY
/// The central hub where all tools are stored and looked up.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }
    
    pub async fn execute_tool(&self, name: &str, args: Value) -> Result<Value, String> {
        if let Some(tool) = self.get(name) {
            tool.execute(args).await
        } else {
            Err(format!("Tool '{}' not found in registry.", name))
        }
    }
}

// ==========================================
// 🛠️ 3. IMPLEMENTING OUR FIRST ACTUAL TOOL
// ==========================================

pub struct SplitClipTool {
    timeline_api: Arc<dyn TimelineApi>,
}

impl SplitClipTool {
    pub fn new(timeline_api: Arc<dyn TimelineApi>) -> Self {
        Self { timeline_api }
    }
}

#[async_trait]
impl Tool for SplitClipTool {
    fn name(&self) -> &'static str {
        "split_clip"
    }

    fn description(&self) -> &'static str {
        "Splits a video or audio clip into two separate clips at a specific timestamp."
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "project_id": { "type": "string", "description": "UUID of the project" },
                "clip_id": { "type": "string", "description": "UUID of the clip to split" },
                "split_time": { "type": "number", "description": "Timestamp in seconds where the split occurs" }
            },
            "required": ["project_id", "clip_id", "split_time"]
        })
    }

    async fn execute(&self, args: Value) -> Result<Value, String> {
        let project_id_str = args["project_id"].as_str().ok_or("Missing project_id")?;
        let clip_id_str = args["clip_id"].as_str().ok_or("Missing clip_id")?;
        let split_time = args["split_time"].as_f64().ok_or("Missing split_time")?;

        let project_id = Uuid::parse_str(project_id_str).map_err(|_| "Invalid project_id UUID")?;
        let clip_id = Uuid::parse_str(clip_id_str).map_err(|_| "Invalid clip_id UUID")?;

        // Safely route the request down into the actual Timeline API
        let (left_id, right_id) = self.timeline_api.split_clip(project_id, clip_id, split_time).await?;

        Ok(json!({
            "status": "success",
            "left_clip_id": left_id.to_string(),
            "right_clip_id": right_id.to_string()
        }))
    }
}