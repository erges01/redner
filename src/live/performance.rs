use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::live::audio::TranscriptTimeline;
use crate::live::motion::MotionTimeline;

// ==========================================
// 1. AI PERFORMANCE MARKERS
// The AI drops these on the timeline in real-time
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarkerType {
    Retake,        // "Let me say that again..."
    Silence,       // Dead air detected
    Emphasis,      // Creator leaned in / raised voice
    AiSuggestion,  // E.g., "Insert Rust Logo here"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMarker {
    pub marker_id: Uuid,
    pub marker_type: MarkerType,
    pub timestamp_ms: u64,
    pub duration_ms: Option<u64>,
    pub description: String,
}

// ==========================================
// 2. THE MASTER PERFORMANCE TIMELINE
// Wraps Audio, Motion, and AI Markers into one object
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorPerformance {
    pub session_id: Uuid,
    pub transcript: TranscriptTimeline,
    pub motion: MotionTimeline,
    pub markers: Vec<PerformanceMarker>,
}

impl CreatorPerformance {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            transcript: TranscriptTimeline::new(session_id),
            motion: MotionTimeline::new(session_id),
            markers: Vec::new(),
        }
    }

    /// The AI uses this to annotate the timeline live
    pub fn add_marker(&mut self, marker_type: MarkerType, timestamp_ms: u64, description: String) {
        let marker = PerformanceMarker {
            marker_id: Uuid::new_v4(),
            marker_type,
            timestamp_ms,
            duration_ms: None,
            description: description.clone(),
        };
        
        println!("📌 [TIMELINE] AI dropped a {:?} marker at {}ms: '{}'", marker.marker_type, timestamp_ms, description);
        self.markers.push(marker);
    }
}