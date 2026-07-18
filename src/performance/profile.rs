use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceProfile {
    pub id: String, 
    pub name: String,
    pub energy: EnergyLevel,
    pub default_expression: ExpressionPreset,
    pub eye_contact_level: u8,
    pub gesture_style: GestureStyle,
    pub preferred_camera: CameraFraming,
    pub speaking_style: SpeakingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakingStyle {
    pub pace: f32,
    pub confidence: String,
    pub pause_length: String,
}

// --- ENUMS ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnergyLevel {
    Low,
    Medium,
    High,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionPreset {
    Neutral,
    Happy,
    Confident,
    Serious,
    Curious,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GestureStyle {
    Minimal,
    Teacher,
    Podcast,
    Energetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")] // To match "close-up", "dynamic-push" etc.
pub enum CameraFraming {
    CloseUp,
    Medium,
    Wide,
    DynamicPush,
}