use uuid::Uuid;
use crate::intelligence::memory::CreativeMemoryStore;
use crate::intelligence::profile::CreatorProfile;
use crate::intelligence::patterns::{DetectedPattern, PatternStatus};

// ==========================================
// THE INTELLIGENCE DASHBOARD CONTROLLER
// Exposes the memory and profile to the React frontend
// ==========================================
pub struct IntelligenceDashboard;

impl IntelligenceDashboard {
    /// Fetch the full Bayesian profile for the `CreativeProfile.tsx` UI
    pub fn get_creator_profile(profile: &CreatorProfile) -> CreatorProfile {
        println!("📊 [DASHBOARD] Fetching Creator Profile for UI render.");
        profile.clone()
    }

    /// The UI calls this when the creator clicks [Approve] on a Detected Pattern
    pub fn approve_pattern(pattern_id: Uuid, patterns_db: &mut Vec<DetectedPattern>) -> Result<(), String> {
        if let Some(pattern) = patterns_db.iter_mut().find(|p| p.pattern_id == pattern_id) {
            pattern.status = PatternStatus::Accepted;
            println!("✅ [DASHBOARD] Creator APPROVED pattern: '{}'", pattern.description);
            // In a full implementation, this would now write a hard rule to the CreatorProfile
            Ok(())
        } else {
            Err("Pattern not found.".to_string())
        }
    }

    /// The UI calls this when the creator clicks [Reject] on a Detected Pattern
    pub fn reject_pattern(pattern_id: Uuid, patterns_db: &mut Vec<DetectedPattern>) -> Result<(), String> {
        if let Some(pattern) = patterns_db.iter_mut().find(|p| p.pattern_id == pattern_id) {
            pattern.status = PatternStatus::Rejected;
            println!("❌ [DASHBOARD] Creator REJECTED pattern: '{}'. The AI will ignore this.", pattern.description);
            Ok(())
        } else {
            Err("Pattern not found.".to_string())
        }
    }

    /// The UI calls this from `MemoryBrowser.tsx` when a creator wants the AI to forget a specific decision
    pub fn forget_decision(decision_id: Uuid, store: &mut CreativeMemoryStore) -> Result<(), String> {
        let initial_len = store.decision_history.len();
        store.decision_history.retain(|d| d.decision_id != decision_id);
        
        if store.decision_history.len() < initial_len {
            println!("🗑️ [DASHBOARD] Memory wiped. Decision {} has been permanently forgotten.", decision_id);
            Ok(())
        } else {
            Err("Decision not found in memory.".to_string())
        }
    }
}