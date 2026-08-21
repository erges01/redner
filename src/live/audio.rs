use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE SPEECH SEGMENT
// A perfectly time-aligned piece of spoken text.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechSegment {
    pub text: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub confidence: f32, // How sure is the STT engine?
}

// ==========================================
// 2. THE TRANSCRIPT TIMELINE
// The running ledger of everything the creator says in the session.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptTimeline {
    pub session_id: Uuid,
    pub segments: Vec<SpeechSegment>,
}

impl TranscriptTimeline {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            segments: Vec::new(),
        }
    }

    /// Appends a new piece of speech directly onto the live ledger
    pub fn append_speech(&mut self, text: String, start_ms: u64, end_ms: u64) {
        let segment = SpeechSegment {
            text: text.clone(),
            start_ms,
            end_ms,
            confidence: 0.95, // Mocked STT confidence
        };
        
        self.segments.push(segment);
        
        println!("🎙️ [AUDIO INTEL] Logged Speech: '{}' [{}ms -> {}ms]", text, start_ms, end_ms);
        
        // 💡 BIG BRAIN ARCHITECTURE:
        // Right here is where we would trigger an event to the Agent System.
        // E.g., if text.contains("let me restart"), we fire a RetakeDetected event!
    }
}