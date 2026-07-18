use serde::{Deserialize, Serialize};

// ==========================================
// 1. INCOMING CONTEXT (From Frontend)
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: String, // "user" or "model"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIClipSnapshot {
    pub id: String,
    pub r#type: String, 
    pub start_ms: f64,            
    pub duration_ms: f64,         
    pub source_offset_ms: f64,    
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AITrackSnapshot {
    pub id: String,
    pub r#type: String,
    pub clips: Vec<AIClipSnapshot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISelectionSnapshot {
    pub selected_clip_id: Option<String>,
    pub selected_track_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIPlaybackSnapshot {
    pub current_time_ms: f64,     
    pub is_playing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIProjectSnapshot {
    pub total_duration_ms: f64,   
    pub track_count: u32,
    pub clip_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIContextSnapshot {
    pub project: AIProjectSnapshot,
    pub tracks: Vec<AITrackSnapshot>,
    pub selection: AISelectionSnapshot,
    pub playback: AIPlaybackSnapshot,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopilotRequest {
    pub prompt: String,
    pub history: Vec<ChatMessage>, // Block D: Conversation tracking added
    pub context: AIContextSnapshot,
}

// ==========================================
// 2. OUTGOING COMMANDS (To/From LLM & Frontend)
// ==========================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AIOperation {
    PlayPause,
    Seek { #[serde(rename = "timeMs")] time_ms: f64 },
    DeleteClip { #[serde(rename = "clipId")] clip_id: String },
    SplitClip { #[serde(rename = "clipId")] clip_id: String, #[serde(rename = "timeMs")] time_ms: f64 },
    
    // 🛠️ NEW EXPANSION PACK
    MoveClip { #[serde(rename = "clipId")] clip_id: String, #[serde(rename = "newStartMs")] new_start_ms: f64 },
    DuplicateClip { #[serde(rename = "clipId")] clip_id: String },
    CreateMarker { #[serde(rename = "timeMs")] time_ms: f64, label: String },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponsePayload {
    pub thoughts: String,
    pub operations: Vec<AIOperation>,
}