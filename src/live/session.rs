use uuid::Uuid;
use crate::live::events::{LiveEvent, LiveSignal};
use crate::live::audio::TranscriptTimeline;
use crate::live::motion::MotionTimeline; // 👈 Import the motion ledger!

pub struct LiveSessionManager {
    pub transcript: TranscriptTimeline,
    pub motion: MotionTimeline, // 👈 Add it to the session state
}

impl LiveSessionManager {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            transcript: TranscriptTimeline::new(session_id),
            motion: MotionTimeline::new(session_id), // 👈 Initialize it
        }
    }

    /// Ingests a high-frequency event from the React frontend
    pub fn process_incoming_signal(&mut self, event: LiveEvent) {
        match event.signal {
            LiveSignal::SessionStarted => {
                println!("🔴 [LIVE] Creator {} started recording in Project {}.", event.creator_id, event.project_id);
            },
            LiveSignal::AudioChunkReceived { chunk_index, byte_size } => {
                println!("🔉 [LIVE] Received Audio Chunk #{} ({} bytes) at {}ms", chunk_index, byte_size, event.session_timestamp_ms);
            },
            LiveSignal::VideoChunkReceived { chunk_index, byte_size } => {
                println!("🎥 [LIVE] Received Video Chunk #{} ({} bytes) at {}ms", chunk_index, byte_size, event.session_timestamp_ms);
            },
            LiveSignal::SpeechDetected { text, start_ms, end_ms } => {
                self.transcript.append_speech(text, start_ms as u64, end_ms as u64);
            },
            LiveSignal::ExpressionDetected { expression, confidence } => {
                // 🛑 NEW: Append the facial expression directly to the motion ledger!
                self.motion.append_expression(event.session_timestamp_ms, expression, confidence);
            },
            LiveSignal::SessionEnded => {
                println!("⏹️ [LIVE] Recording stopped. Finalizing assets...");
            }
        }
    }
}