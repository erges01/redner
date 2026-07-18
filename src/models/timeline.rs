use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineDocument {
    pub id: String,
    pub project_id: Uuid,
    pub fps: u32,
    pub duration_ms: u64,
    pub zoom: f32,
    pub playhead_ms: u64,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub r#type: TrackType,
    pub order: i32,
    pub muted: Option<bool>,
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackType {
    Video,
    Audio,
    Text,
    Script,
    Caption,
    Avatar,
    Overlay,
    Performance, // 👈 NEW: Director Track
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub track_id: String,
    pub r#type: ClipType,
    pub start_ms: u64,
    pub duration_ms: u64,
    pub source_offset_ms: Option<u64>,
    pub asset_id: Option<Uuid>,
    pub label: Option<String>,
    pub color: Option<String>,
    pub content: Option<ClipContent>,
    pub metadata: Option<serde_json::Value>, // 👈 NEW: Catch the Performance JSON payload
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipType {
    Video,
    Audio,
    Image,
    Text,
    ScriptSegment,
    Caption,
    AvatarSegment,
    Gap,
    PerformanceInstruction, // 👈 NEW: Serializes to "performance_instruction"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ClipContent {
    Video(VideoClipContent),
    Audio(AudioClipContent),
    Text(TextClipContent),
    Script(ScriptClipContent),
    Caption(CaptionClipContent),
    Avatar(AvatarClipContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoClipContent {
    pub transform: Option<Transform>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioClipContent {
    pub volume: f32,
    pub fade_in_ms: Option<u64>,
    pub fade_out_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextClipContent {
    pub text: String,
    pub font_size: Option<u32>,
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptClipContent {
    pub text: String,
    pub speaker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionClipContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarClipContent {
    pub avatar_id: Option<String>,
    pub expression: Option<String>,
    pub pose: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}