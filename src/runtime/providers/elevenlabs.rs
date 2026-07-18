use std::path::PathBuf;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;
use tokio::io::AsyncWriteExt;

use crate::runtime::providers::voice::{VoiceProvider, VoiceGenerationParams};

pub struct ElevenLabsProvider {
    client: Client,
    api_key: String,
}

impl ElevenLabsProvider {
    pub fn new() -> Self {
        let api_key = env::var("ELEVENLABS_API_KEY")
            .expect("ELEVENLABS_API_KEY must be set in .env");
        
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl VoiceProvider for ElevenLabsProvider {
    async fn generate_audio(
        &self, 
        params: VoiceGenerationParams, 
        output_path: PathBuf
    ) -> Result<(), String> {
        
        // Change the default voice_id to a Base voice (Adam)
        let voice_id = if params.voice_id == "default_voice" || params.voice_id.is_empty() {
            "pNInz6obpgDQGcFmaJgB".to_string() 
        } else {
            params.voice_id
        };

        let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{}", voice_id);

        let body = json!({
            "text": params.text,
            "model_id": "eleven_multilingual_v2", // 👈 The bulletproof model
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.5
            }
        });

        // 1. Make the API Call
        let response = self.client.post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Accept", "audio/mpeg")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("ElevenLabs API error: {}", error_text));
        }

        // 2. Download the audio bytes
        let audio_bytes = response.bytes().await
            .map_err(|e| format!("Failed to read audio bytes: {}", e))?;

        // 3. Save to our local workspace
        let mut file = tokio::fs::File::create(&output_path).await
            .map_err(|e| format!("Failed to create audio file: {}", e))?;
            
        file.write_all(&audio_bytes).await
            .map_err(|e| format!("Failed to write audio bytes to file: {}", e))?;

        Ok(())
    }
}