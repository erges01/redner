use serde::{Deserialize, Serialize};
use crate::intelligence::profile::CreatorProfile;

// ==========================================
// 1. THE STYLE SUMMARY
// Concrete metrics extracted mathematically from a finished timeline.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleSummary {
    pub average_shot_duration_ms: u32,
    pub pacing_category: String,       // "fast", "medium", "slow"
    pub b_roll_frequency_per_min: f32, // B-roll scenes per minute
    pub transition_preference: String, // Most frequently used transition type
}

// ==========================================
// 2. THE STYLE ANALYZER ENGINE
// ==========================================
pub struct StyleAnalyzer;

impl StyleAnalyzer {
    /// Simulates analyzing a rendered timeline to extract concrete style metrics.
    /// In production, this iterates over the actual Redner Runtime Graph (Phase 6).
    pub fn extract_style_metrics(
        timeline_duration_ms: u64, 
        cut_count: u32, 
        b_roll_count: u32
    ) -> StyleSummary {
        // 1. Calculate Average Shot Length
        let avg_shot_duration = if cut_count > 0 {
            (timeline_duration_ms / (cut_count as u64 + 1)) as u32
        } else {
            timeline_duration_ms as u32
        };

        // 2. Categorize the pacing based on cinematic standards
        let pacing = match avg_shot_duration {
            0..=2500 => "fast",     // Under 2.5s per shot (TikTok/Reels style)
            2501..=5000 => "medium",// 2.5s - 5s per shot (Standard YouTube)
            _ => "slow",            // 5s+ per shot (Documentary/Cinematic)
        };

        // 3. Calculate Visual Density (B-Roll per minute)
        let minutes = (timeline_duration_ms as f32) / 60000.0;
        let b_roll_freq = if minutes > 0.0 {
            b_roll_count as f32 / minutes
        } else {
            0.0
        };

        StyleSummary {
            average_shot_duration_ms: avg_shot_duration,
            pacing_category: pacing.to_string(),
            b_roll_frequency_per_min: b_roll_freq,
            transition_preference: "hard_cut".to_string(), // Defaulting based on dev-focused style
        }
    }

    /// Bridges Phase 11.3 (Extraction) with Phase 11.2 (Bayesian Profile)
    pub fn apply_to_profile(summary: &StyleSummary, profile: &mut CreatorProfile, timestamp_ms: u64) {
        println!("--------------------------------------------------");
        println!("📊 [STYLE INTELLIGENCE] Extracted Timeline Metrics:");
        println!("   ├─ Pacing Category: {}", summary.pacing_category);
        println!("   ├─ Avg Shot Duration: {}ms", summary.average_shot_duration_ms);
        println!("   └─ Visual Density: {:.1} B-Rolls/min", summary.b_roll_frequency_per_min);
        
        // Feed the extracted pacing back into the Bayesian profile engine!
        // If the creator makes 5 "fast" videos in a row, the confidence score will approach 100%.
        let pacing_key = format!("{}_pacing", summary.pacing_category);
        profile.register_editing_preference(&pacing_key, timestamp_ms);
        
        let visual_key = if summary.b_roll_frequency_per_min > 4.0 { "high_visual_density" } else { "low_visual_density" };
        
        let entry = profile.visual_style.entry(visual_key.to_string()).or_insert_with(|| {
            crate::intelligence::profile::PreferenceScore {
                value: visual_key.to_string(),
                confidence: 0.0,
                evidence_count: 0,
                last_observed_ms: timestamp_ms,
            }
        });
        entry.observe(timestamp_ms);
        
        println!("🧠 [STYLE INTELLIGENCE] Metrics successfully mapped to Creator DNA.");
    }
}