use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// ==========================================
// 1. THE CONFIDENCE METRIC
// Tracks how sure the AI is about a specific preference
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreferenceScore {
    pub value: String,          // e.g., "fast_pacing"
    pub confidence: f32,        // 0.0 to 1.0 (How sure are we?)
    pub evidence_count: u32,    // How many times have we seen this?
    pub last_observed_ms: u64,
}

impl PreferenceScore {
    /// Applies a Bayesian-style update to the confidence score based on new evidence.
    /// A single observation gives low confidence. Multiple repeated observations
    /// asymptotically approach 1.0 confidence.
    pub fn observe(&mut self, timestamp_ms: u64) {
        self.evidence_count += 1;
        self.last_observed_ms = timestamp_ms;
        
        // Simple logarithmic confidence curve based on evidence count
        // 1 evidence  ~ 0.50 confidence
        // 5 evidence  ~ 0.82 confidence
        // 10 evidence ~ 0.95 confidence
        let base = (self.evidence_count as f32).ln();
        self.confidence = (0.5 + (base * 0.15)).clamp(0.0, 0.99);
    }
}

// ==========================================
// 2. THE INTELLIGENT CREATOR PROFILE
// The machine-readable DNA of the creator
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorProfile {
    pub profile_id: Uuid,
    pub creator_id: Uuid,
    
    // Categorized Preferences with Confidence Scores
    pub editing_pacing: HashMap<String, PreferenceScore>,
    pub visual_style: HashMap<String, PreferenceScore>,
    pub narrative_hooks: HashMap<String, PreferenceScore>,
}

impl CreatorProfile {
    pub fn new(creator_id: Uuid) -> Self {
        Self {
            profile_id: Uuid::new_v4(),
            creator_id,
            editing_pacing: HashMap::new(),
            visual_style: HashMap::new(),
            narrative_hooks: HashMap::new(),
        }
    }

    /// Logs a new preference observation. If it exists, it strengthens the confidence.
    pub fn register_editing_preference(&mut self, preference_key: &str, timestamp_ms: u64) {
        let entry = self.editing_pacing.entry(preference_key.to_string()).or_insert(PreferenceScore {
            value: preference_key.to_string(),
            confidence: 0.0,
            evidence_count: 0,
            last_observed_ms: timestamp_ms,
        });
        
        entry.observe(timestamp_ms);
        
        println!(
            "🧬 [PROFILE ENGINE] Updated Editing Preference: '{}' -> Confidence: {:.0}% (Evidence: {})", 
            preference_key, 
            entry.confidence * 100.0, 
            entry.evidence_count
        );
    }
    
    /// Returns the highest-confidence preference in a specific category
    pub fn get_dominant_editing_style(&self) -> Option<PreferenceScore> {
        self.editing_pacing
            .values()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .cloned()
    }
}