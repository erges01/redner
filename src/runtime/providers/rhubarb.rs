use std::path::PathBuf;
use async_trait::async_trait;
use tokio::process::Command;
use crate::runtime::providers::lipsync::LipSyncProvider;

pub struct RhubarbProvider;

impl RhubarbProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LipSyncProvider for RhubarbProvider {
    async fn generate_visemes(
        &self, 
        audio_path: PathBuf, 
        output_path: PathBuf
    ) -> Result<(), String> {
        
        // 1. Convert the .mp3 to .wav instantly using FFmpeg
        let wav_path = audio_path.with_extension("wav");
        
        println!("   ⚙️ [Rhubarb] Converting MP3 to WAV for analysis...");
        let convert_output = Command::new("./ffmpeg.exe")
            .arg("-y") // Overwrite if exists
            .arg("-i").arg(&audio_path)
            .arg(&wav_path)
            .output()
            .await
            .map_err(|e| format!("FFmpeg missing! Open PowerShell as Admin, run 'winget install ffmpeg', then restart VS Code. Error: {}", e))?;

        if !convert_output.status.success() {
            return Err("Failed to convert MP3 to WAV. Is FFmpeg installed?".to_string());
        }

        // 2. Run Rhubarb on the new .wav file
        println!("   ⚙️ [Rhubarb] Running AI lip-sync analysis...");
        let output = Command::new("./rhubarb.exe")
            .arg("-f").arg("json")
            .arg("-o").arg(&output_path)
            .arg(&wav_path)
            .output()
            .await
            .map_err(|e| format!("Failed to run Rhubarb executable: {}", e))?;

        if !output.status.success() {
            let err_str = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Rhubarb processing failed: {}", err_str));
        }

        Ok(())
    }
}