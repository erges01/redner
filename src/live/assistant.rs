use crate::live::performance::{CreatorPerformance, MarkerType};

// ==========================================
// THE LIVE AI ASSISTANT
// Watches the stream in real-time and annotates the timeline.
// ==========================================
pub struct LiveAssistant;

impl LiveAssistant {
    /// Analyzes the spoken text for verbal cues (retakes, emphasis, topics)
    pub fn analyze_speech(text: &str, timestamp_ms: u64, performance: &mut CreatorPerformance) {
        let lower_text = text.to_lowercase();
        
        // 1. Detect Retakes (The creator messed up and wants to restart a sentence)
        if lower_text.contains("wait let me") || 
           lower_text.contains("scratch that") || 
           lower_text.contains("let me say that again") {
            
            println!("🤖 [LIVE ASSISTANT] Retake phrase detected: '{}'", text);
            performance.add_marker(
                MarkerType::Retake,
                timestamp_ms,
                "Creator verbally signaled a retake. Suggesting a cut.".to_string(),
            );
        }

        // 2. Detect Content Topics (For real-time B-Roll suggestions)
        if lower_text.contains("ownership system") || lower_text.contains("borrow checker") {
            println!("🤖 [LIVE ASSISTANT] Technical concept detected: '{}'", text);
            performance.add_marker(
                MarkerType::AiSuggestion,
                timestamp_ms,
                "Suggesting B-Roll: Rust Memory Management Diagram".to_string(),
            );
        }
    }

    /// Analyzes the time between words to detect dead air
    pub fn analyze_silence(last_end_ms: u64, current_start_ms: u64, performance: &mut CreatorPerformance) {
        let gap = current_start_ms.saturating_sub(last_end_ms);
        
        // If the creator pauses for more than 3 seconds (3000ms)
        if gap > 3000 { 
            println!("🤖 [LIVE ASSISTANT] Dead air detected ({}ms).", gap);
            performance.add_marker(
                MarkerType::Silence,
                last_end_ms, // The silence started when they stopped speaking
                format!("{}ms of silence detected. Suggesting automatic trim.", gap),
            );
        }
    }
}