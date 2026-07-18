use std::path::PathBuf;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct VoiceGenerationParams {
    pub text: String,
    pub voice_id: String,
    pub speed: f32,
}

/// The universal interface for any TTS Engine (ElevenLabs, OpenAI, PlayHT, etc.)
#[async_trait]
pub trait VoiceProvider: Send + Sync {
    /// Takes text and configuration, generates audio, and saves it to the given path
    async fn generate_audio(
        &self, 
        params: VoiceGenerationParams, 
        output_path: PathBuf
    ) -> Result<(), String>;
}