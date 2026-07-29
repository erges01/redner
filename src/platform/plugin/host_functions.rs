use extism::host_fn;
use uuid::Uuid;
use std::sync::Arc;
use serde_json::{Value, json};

use crate::platform::api::TimelineApi;

pub struct PluginContext {
    pub timeline_api: Arc<dyn TimelineApi>,
}

host_fn!(pub timeline_split_clip(user_data: PluginContext; payload: String) -> String {
    let args: Value = serde_json::from_str(&payload).unwrap_or_default();
    
    let project_id_str = args["project_id"].as_str().unwrap_or_default();
    let clip_id_str = args["clip_id"].as_str().unwrap_or_default();
    let split_time = args["split_time"].as_f64().unwrap_or(0.0);

    let project_id = Uuid::parse_str(project_id_str).unwrap_or_default();
    let clip_id = Uuid::parse_str(clip_id_str).unwrap_or_default();

    // 1. Get the secure context from the Wasm boundary
    let context = user_data.get()?;

    // 2. Safely bridge synchronous Wasm to asynchronous Rust
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            // 3. Lock the mutex and call our Core API!
            let locked_context = context.lock().unwrap();
            locked_context.timeline_api.split_clip(project_id, clip_id, split_time).await
        })
    });

    // 4. Return the result to the Wasm plugin
    match result {
        Ok((left_id, right_id)) => {
            Ok(json!({
                "status": "success",
                "left_clip_id": left_id.to_string(),
                "right_clip_id": right_id.to_string()
            }).to_string())
        },
        Err(e) => {
            Ok(json!({
                "status": "error",
                "message": e
            }).to_string())
        }
    }
});