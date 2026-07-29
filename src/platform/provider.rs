use async_trait::async_trait;

/// The strict contract that ANY AI model must follow to plug into Redner.
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Identifies the provider (e.g., "ElevenLabs", "OpenAI", "Anthropic")
    fn name(&self) -> &'static str;

    /// Generates text or JSON Blueprints (LLMs)
    async fn generate_text(&self, prompt: &str, system_prompt: Option<&str>) -> Result<String, String>;

    /// Generates audio bytes from text (TTS)
    async fn generate_speech(&self, text: &str, voice_profile_id: &str) -> Result<Vec<u8>, String>;
    
    /// (Future-proofing) Generates a video scene from a prompt
    async fn generate_scene(&self, _prompt: &str) -> Result<Vec<u8>, String> {
        Err("Scene generation not supported by this provider yet.".to_string())
    }
}