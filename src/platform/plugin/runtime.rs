use extism::{Plugin, Manifest, Wasm, Function};
use std::path::PathBuf;
use serde_json::Value;
use std::sync::Arc;

use crate::platform::api::TimelineApi;
use crate::platform::plugin::host_functions::{PluginContext, timeline_split_clip};

pub struct PluginRuntime {
    plugin: Plugin,
    pub id: String,
    pub name: String,
}

impl PluginRuntime {
    /// Loads a WebAssembly plugin and injects our Host Functions (the Redner API) into it.
    pub fn load_from_file(
        plugin_id: &str, 
        plugin_name: &str, 
        wasm_path: PathBuf,
        timeline_api: Arc<dyn TimelineApi>, // 👈 We pass the Core API in when loading the plugin
    ) -> Result<Self, String> {
        
        let manifest = Manifest::new([Wasm::file(wasm_path)]);
        
        // 1. Bundle our APIs into the Plugin Context
        let context = PluginContext {
            timeline_api,
        };

        // 2. Register the Host Functions that the Wasm plugin is allowed to call
        let f_split_clip = Function::new(
            "timeline_split_clip", 
            [extism::ValType::I64], // Pointer to input string
            [extism::ValType::I64], // Pointer to output string
            extism::UserData::new(context), // FIX 4: Removed the Some() wrapper
            timeline_split_clip
        );

        // 3. Initialize the sandbox and inject the host functions!
        let plugin = Plugin::new(&manifest, [f_split_clip], true)
            .map_err(|e| format!("Failed to initialize Wasm sandbox: {}", e))?;

        println!("   📦 [PluginRuntime] Successfully loaded Wasm plugin: {}", plugin_name);

        Ok(Self {
            plugin,
            id: plugin_id.to_string(),
            name: plugin_name.to_string(),
        })
    }

    /// Executes a specific function exported by the WebAssembly plugin.
    pub fn execute(&mut self, function_name: &str, payload: Value) -> Result<Value, String> {
        let input_string = payload.to_string();
        
        let output_string: String = self.plugin
            .call(function_name, input_string)
            .map_err(|e| format!("Plugin execution failed: {}", e))?;

        let output_json: Value = serde_json::from_str(&output_string)
            .map_err(|e| format!("Plugin returned invalid JSON: {}", e))?;

        Ok(output_json)
    }
}