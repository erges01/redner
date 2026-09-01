use serde::{Deserialize, Serialize};

// ==========================================
// 1. PROVIDER DTOs
// Standardized inputs and outputs for ALL media models.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaResponse {
    pub asset_url: String,        // Where the generated file lives
    pub mime_type: String,        // e.g., "audio/mpeg", "video/mp4"
    pub size_bytes: u64,
    pub duration_ms: Option<u64>, // For audio/video
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceRequest {
    pub text: String,
    pub voice_id: String,
    pub language: Option<String>, // e.g., "en-NG" (Nigerian English)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VisualRequest {
    pub prompt: String,
    pub aspect_ratio: String, // e.g., "9:16"
    pub style: Option<String>,
}

// ==========================================
// 2. THE MEDIA PROVIDER TRAITS
// Third-party developers implement these to inject their models into Redner.
// ==========================================

pub trait VoiceProvider {
    /// The unique identifier for this provider (e.g., "com.elevenlabs.voice")
    fn provider_id(&self) -> &str;
    
    /// The core generation method
    fn generate_speech(&self, request: &VoiceRequest) -> Result<MediaResponse, String>;
}

pub trait VisualProvider {
    fn provider_id(&self) -> &str;
    fn generate_image(&self, request: &VisualRequest) -> Result<MediaResponse, String>;
    fn generate_video(&self, request: &VisualRequest) -> Result<MediaResponse, String>;
}