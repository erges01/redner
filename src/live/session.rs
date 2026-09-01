use uuid::Uuid;
use crate::live::events::{LiveEvent, LiveSignal};
use crate::live::performance::CreatorPerformance;
use crate::live::assistant::LiveAssistant; // 👈 Import the Assistant!

pub struct LiveSessionManager {
    pub performance: CreatorPerformance,
}

impl LiveSessionManager {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            performance: CreatorPerformance::new(session_id),
        }
    }

    pub fn process_incoming_signal(&mut self, event: LiveEvent) {
        match event.signal {
            LiveSignal::SessionStarted => {
                println!("🔴 [LIVE] Creator {} started recording.", event.creator_id);
            },
            LiveSignal::AudioChunkReceived { chunk_index, byte_size } => {
                // Audio streaming...
            },
            LiveSignal::VideoChunkReceived { chunk_index, byte_size } => {
                // Video streaming...
            },
            LiveSignal::SpeechDetected { text, start_ms, end_ms } => {
                // 1. Check for silence between the last word and this new word
                if let Some(last_speech) = self.performance.transcript.segments.last() {
                    LiveAssistant::analyze_silence(last_speech.end_ms, start_ms as u64, &mut self.performance);
                }

                // 2. Check the text for verbal cues and topics
                LiveAssistant::analyze_speech(&text, start_ms as u64, &mut self.performance);

                // 3. Finally, save the transcript
                self.performance.transcript.append_speech(text, start_ms as u64, end_ms as u64);
            },
            LiveSignal::ExpressionDetected { expression, confidence } => {
                self.performance.motion.append_expression(event.session_timestamp_ms, expression, confidence);
            },
            LiveSignal::SessionEnded => {
                println!("⏹️ [LIVE] Recording stopped. Master Performance Timeline saved.");
            }
        }
    }
}