use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// ==========================================
// 1. SPATIAL DATA MODELS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinate {
    pub x: f32, // Normalized 0.0 to 1.0
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeadPose {
    pub pitch: f32, // Up/Down
    pub yaw: f32,   // Left/Right
}

// ==========================================
// 2. THE MOTION FRAME
// A single snapshot of the creator's physical state.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MotionFrame {
    pub timestamp_ms: u64,
    pub face_position: Option<Coordinate>,
    pub head_pose: Option<HeadPose>,
    pub expressions: HashMap<String, f32>, // e.g., "smile" -> 0.72, "surprise" -> 0.85
}

// ==========================================
// 3. THE MOTION TIMELINE
// The running ledger of the creator's physical performance.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MotionTimeline {
    pub session_id: Uuid,
    pub frames: Vec<MotionFrame>,
}

impl MotionTimeline {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            frames: Vec::new(),
        }
    }

    /// Logs an expression detected by the frontend camera tracker
    pub fn append_expression(&mut self, timestamp_ms: u64, expression: String, confidence: f32) {
        let mut expressions = HashMap::new();
        expressions.insert(expression.clone(), confidence);

        let frame = MotionFrame {
            timestamp_ms,
            // These will be fully populated when we hook up a facial tracker in the browser!
            face_position: None, 
            head_pose: None,
            expressions,
        };

        self.frames.push(frame);
        println!("🎭 [MOTION INTEL] Logged Expression '{}' ({}%) at {}ms", expression, (confidence * 100.0).round(), timestamp_ms);
    }
}