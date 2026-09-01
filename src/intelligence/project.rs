use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::intelligence::profile::CreatorProfile;
use crate::intelligence::memory::ProjectMemory;

// ==========================================
// 1. THE PROJECT INTENT
// What is the underlying goal of this specific timeline?
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectIntent {
    pub primary_goal: String,      // e.g., "Educate", "Entertain", "Sell", "Vlog"
    pub emotional_tone: String,    // e.g., "Empathetic", "Hype", "Analytical"
    pub complexity_level: String,  // e.g., "Beginner", "Expert"
}

// ==========================================
// 2. THE PROJECT INTELLIGENCE ENGINE
// The brain that evaluates agent actions against project goals.
// ==========================================
pub struct ProjectIntelligence;

impl ProjectIntelligence {
    /// Evaluates if a proposed creative action fits the current project's constraints and audience.
    pub fn evaluate_creative_fit(
        proposed_action: &str, 
        project_memory: &ProjectMemory, 
        intent: &ProjectIntent
    ) -> Result<(), String> {
        
        println!("🧠 [PROJECT INTEL] Evaluating proposal: '{}'", proposed_action);
        println!("   ├─ Audience: {}", project_memory.target_audience);
        println!("   └─ Goal: {} ({})", intent.primary_goal, intent.emotional_tone);

        // Simulated Intelligence Logic
        let action_lower = proposed_action.to_lowercase();
        let is_expert_audience = project_memory.target_audience.to_lowercase().contains("engineer") || 
                                 intent.complexity_level == "Expert";

        // If the agent proposes something overly basic for an expert audience
        if is_expert_audience && action_lower.contains("explain like i'm 5") {
            return Err("Proposal rejected: Tone is too juvenile for an expert engineering audience.".to_string());
        }

        // If the agent proposes flashy memes for an analytical project
        if intent.emotional_tone == "Analytical" && action_lower.contains("dank meme") {
            return Err("Proposal rejected: Meme visuals clash with the Analytical tone of this project.".to_string());
        }

        println!("✅ [PROJECT INTEL] Proposal aligns perfectly with project intent.");
        Ok(())
    }

    /// Derives project-specific constraints by merging the active project with the Creator's global profile.
    pub fn derive_active_constraints(
        project_memory: &ProjectMemory, 
        creator_profile: &CreatorProfile
    ) -> Vec<String> {
        let mut constraints = project_memory.active_constraints.clone();
        
        // If the creator strongly prefers fast pacing globally, enforce it on the project
        if let Some(pacing) = creator_profile.get_dominant_editing_style() {
            if pacing.confidence > 0.8 {
                constraints.push(format!("Enforce global pacing rule: {}", pacing.value));
            }
        }
        
        constraints
    }
}