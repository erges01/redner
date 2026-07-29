use std::sync::Arc;
use std::path::PathBuf;
use serde_json::json;
use uuid::Uuid;
use async_trait::async_trait;

use crate::platform::api::TimelineApi;
use crate::platform::plugin::PluginRuntime;

// 1. Create a Fake Core Engine for testing
struct FakeCoreEngine;

#[async_trait]
impl TimelineApi for FakeCoreEngine {
    // 1. The methods the trait requires that we aren't testing right now (we just return Ok)
    async fn add_track(&self, _project_id: Uuid, _name: &str) -> Result<Uuid, String> { Ok(Uuid::new_v4()) }
    async fn remove_track(&self, _project_id: Uuid, _track_id: Uuid) -> Result<(), String> { Ok(()) }
    async fn move_clip(&self, _project_id: Uuid, _clip_id: Uuid, _new_start_time: f64) -> Result<(), String> { Ok(()) }
    async fn delete_clip(&self, _project_id: Uuid, _clip_id: Uuid) -> Result<(), String> { Ok(()) }
    
    // 2. THIS IS THE FUNCTION THE WASM PLUGIN WILL ACTUALLY CALL!
    async fn split_clip(&self, _project_id: Uuid, clip_id: Uuid, split_time: f64) -> Result<(Uuid, Uuid), String> {
        println!("   ⚙️ [CORE ENGINE] The Wasm sandbox just reached across the bridge!");
        println!("   ⚙️ [CORE ENGINE] Splitting Clip: {} at {} seconds.", clip_id, split_time);
        
        // We simulate a successful split by returning two brand new Clip UUIDs
        Ok((Uuid::new_v4(), Uuid::new_v4()))
    }
}

pub async fn run_wasm_demo() {
    println!("\n🚀 Booting Redner Wasm Plugin Demo...");

    let core_api = Arc::new(FakeCoreEngine);
    
    // Path to the Wasm file we just built
    let wasm_path = PathBuf::from("../redner-auto-cutter/target/wasm32-unknown-unknown/release/redner_auto_cutter.wasm");

    let mut plugin = match PluginRuntime::load_from_file("plug_auto_cut", "Auto-Cutter", wasm_path, core_api) {
        Ok(p) => p,
        Err(e) => {
            println!("   ❌ [HOST] Failed to load Wasm plugin: {}", e);
            return;
        }
    };

    println!("   ▶️ [HOST] Handing control over to the Wasm sandbox...");
    
    let payload = json!({
        "project_id": Uuid::new_v4().to_string(),
        "clip_id": Uuid::new_v4().to_string(),
    });

    match plugin.execute("run_auto_cutter", payload) {
        Ok(result) => {
            println!("   ✅ [HOST] Wasm Sandbox returned control successfully!");
            println!("   📦 [HOST] Final Output from Plugin:\n{:#?}\n", result);
        }
        Err(e) => {
            println!("   ❌ [HOST] Plugin Execution Failed: {}\n", e);
        }
    }
}