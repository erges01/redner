use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE LIVE SIGNAL PAYLOADS
// What the browser sends to Rust in real-time
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum LiveSignal {
    /// Fired when the creator hits "START RECORDING"
    SessionStarted,
    
    /// The browser sends a 2-second chunk of audio
    AudioChunkReceived { chunk_index: u32, byte_size: usize },
    
    /// The browser sends a 2-second chunk of video
    VideoChunkReceived { chunk_index: u32, byte_size: usize },
    
    /// (Phase 10.2 preview) The browser STT recognized words
    SpeechDetected { text: String, start_ms: u32, end_ms: u32 },
    
    /// (Phase 10.3 preview) The browser detected a smile or frown
    ExpressionDetected { expression: String, confidence: f32 },
    
    /// Fired when the creator hits "STOP RECORDING"
    SessionEnded,
}

// ==========================================
// 2. THE LIVE EVENT ENVELOPE
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiveEvent {
    pub event_id: Uuid,
    pub creator_id: Uuid,
    pub project_id: Uuid,
    pub session_timestamp_ms: u64, // Milliseconds since the session started
    pub signal: LiveSignal,
}