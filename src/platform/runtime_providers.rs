use async_trait::async_trait;
use std::process::Command;
use std::path::PathBuf;
use serde_json::{json, Value};

/// 1. THE LIPSYSNC CONTRACT
/// Any lip-sync engine (Rhubarb, SyncLabs, Wav2Lip) must follow this interface.
#[async_trait]
pub trait LipSyncProvider: Send + Sync {
    fn name(&self) -> &'static str;
    
    /// Accepts an audio path and dialogue text, returns mouth cue timing JSON
    async fn generate_cues(&self, audio_path: PathBuf, text: &str) -> Result<Value, String>;
}

/// 2. THE RHUBARB CLI IMPLEMENTATION
pub struct RhubarbProvider {
    executable_path: PathBuf,
}

impl RhubarbProvider {
    pub fn new(executable_path: PathBuf) -> Self {
        Self { executable_path }
    }
}

#[async_trait]
impl LipSyncProvider for RhubarbProvider {
    fn name(&self) -> &'static str {
        "Rhubarb LipSync CLI"
    }

    async fn generate_cues(&self, audio_path: PathBuf, _text: &str) -> Result<Value, String> {
        let output_json_path = audio_path.with_extension("json");

        // Shell out to the local Rhubarb binary process asynchronously
        let exec_path = self.executable_path.clone();
        let audio_p = audio_path.clone();
        let out_p = output_json_path.clone();

        let result = tokio::task::spawn_blocking(move || {
            Command::new(exec_path)
                .arg("-a")
                .arg(audio_p)
                .arg("-f")
                .arg("json")
                .arg("-o")
                .arg(&out_p)
                .output()
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Failed to execute Rhubarb process: {}", e))?;

        if result.status.success() {
            // Read generated mouth cues JSON
            let json_content = std::fs::read_to_string(&output_json_path)
                .map_err(|e| format!("Failed to read mouth cues output: {}", e))?;
            
            let cues: Value = serde_json::from_str(&json_content)
                .map_err(|e| format!("Failed to parse mouth cues JSON: {}", e))?;

            Ok(cues)
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr);
            Err(format!("Rhubarb CLI error: {}", stderr))
        }
    }
}