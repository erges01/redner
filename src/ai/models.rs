use serde::{Deserialize, Serialize};

// ==========================================
// 1. INCOMING CONTEXT (From Frontend)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIClipSnapshot {
    pub id: String,
    pub r#type: String, 
    pub start_ms: f64,            // 🛠️ FIX: Changed to f64 to accept decimals
    pub duration_ms: f64,         // 🛠️ FIX: Changed to f64
    pub source_offset_ms: f64,    // 🛠️ FIX: Changed to f64
    pub label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AITrackSnapshot {
    pub id: String,
    pub r#type: String,
    pub clips: Vec<AIClipSnapshot>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISelectionSnapshot {
    pub selected_clip_id: Option<String>,
    pub selected_track_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIPlaybackSnapshot {
    pub current_time_ms: f64,     // 🛠️ FIX: Changed to f64
    pub is_playing: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIProjectSnapshot {
    pub total_duration_ms: f64,   // 🛠️ FIX: Changed to f64
    pub track_count: u32,
    pub clip_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIContextSnapshot {
    pub project: AIProjectSnapshot,
    pub tracks: Vec<AITrackSnapshot>,
    pub selection: AISelectionSnapshot,
    pub playback: AIPlaybackSnapshot,
}

#[derive(Debug, Deserialize)]
pub struct CopilotRequest {
    pub prompt: String,
    pub context: AIContextSnapshot,
}

// ==========================================
// 2. OUTGOING COMMANDS (To Frontend)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AIOperation {
    PlayPause,
    Seek { #[serde(rename = "timeMs")] time_ms: f64 },
    DeleteClip { #[serde(rename = "clipId")] clip_id: String },
    SplitClip { #[serde(rename = "clipId")] clip_id: String, #[serde(rename = "timeMs")] time_ms: f64 },
}

#[derive(Debug, Serialize)]
pub struct AIResponsePayload {
    pub thoughts: String,
    pub operations: Vec<AIOperation>,
}