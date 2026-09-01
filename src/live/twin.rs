use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE IDENTITY PROFILES
// Modular components of a Creator's identity.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FaceProfile {
    pub profile_id: Uuid,
    pub base_image_urls: Vec<String>,     // Reference photos
    pub facial_mesh_id: Option<String>,   // 3D mesh for performance mapping
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceProfile {
    pub profile_id: Uuid,
    pub acoustic_model_id: String,        // ElevenLabs or custom local model ID
    pub base_language: String,            // e.g., "en-NG" (Nigerian English)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MotionProfile {
    pub profile_id: Uuid,
    pub common_gestures: Vec<String>,     // e.g., ["hand_chop", "leaning_forward"]
    pub baseline_energy: f32,             // 0.0 to 1.0 (chill vs highly animated)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceProfile {
    pub profile_id: Uuid,
    pub speech_wpm: u32,                  // Average words per minute
    pub avg_pause_ms: u32,                // Standard pause duration between thoughts
}

// ==========================================
// 2. THE MASTER DIGITAL TWIN
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DigitalTwin {
    pub twin_id: Uuid,
    pub creator_id: Uuid,
    pub name: String,
    
    // Modular sub-profiles
    pub face: Option<FaceProfile>,
    pub voice: Option<VoiceProfile>,
    pub motion: Option<MotionProfile>,
    pub performance: Option<PerformanceProfile>,
}

impl DigitalTwin {
    /// Initializes a blank Digital Twin for a new Creator.
    /// Over time, live sessions will populate these profiles.
    pub fn new(creator_id: Uuid, name: &str) -> Self {
        Self {
            twin_id: Uuid::new_v4(),
            creator_id,
            name: name.to_string(),
            face: None,
            voice: None,
            motion: None,
            performance: None,
        }
    }
}