use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::platform::provider::AiProvider;

pub struct ElevenLabsProvider {
    client: Client,
    api_key: String,
}

impl ElevenLabsProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl AiProvider for ElevenLabsProvider {
    fn name(&self) -> &'static str {
        "ElevenLabs"
    }

    async fn generate_text(&self, _prompt: &str, _system_prompt: Option<&str>) -> Result<String, String> {
        // ElevenLabs is strictly a voice platform, not an LLM.
        Err("ElevenLabs only supports Text-to-Speech (generate_speech), not text generation.".to_string())
    }

    async fn generate_speech(&self, text: &str, voice_profile_id: &str) -> Result<Vec<u8>, String> {
        // This is the actual ElevenLabs REST API endpoint
        let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{}", voice_profile_id);
        
        let payload = json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75
            }
        });

        // Make the asynchronous HTTP POST request
        let response = self.client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("ElevenLabs network error: {}", e))?;

        if response.status().is_success() {
            // Read the raw MP3 audio bytes from the response
            let bytes = response.bytes().await.map_err(|e| format!("Failed to read audio bytes: {}", e))?;
            Ok(bytes.to_vec())
        } else {
            let err_text = response.text().await.unwrap_or_default();
            Err(format!("ElevenLabs API Error: {}", err_text))
        }
    }
}