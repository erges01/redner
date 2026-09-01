use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::live::performance::{CreatorPerformance, MarkerType};

// ==========================================
// 1. THE SCENE PROPOSAL
// A concrete visual suggestion waiting for human approval.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneProposal {
    pub proposal_id: Uuid,
    pub timestamp_ms: u64,
    pub visual_asset_id: String, // e.g., "diagram_rust_memory"
    pub layout: String,          // e.g., "PictureInPicture", "Fullscreen", "SplitScreen"
    pub description: String,
    pub is_approved: bool,       // 🛑 The Gatekeeper lock
}

// ==========================================
// 2. THE LIVE ASSEMBLER
// Turns abstract markers into concrete visual proposals.
// ==========================================
pub struct LiveAssembler;

impl LiveAssembler {
    /// Scans the performance timeline and translates AI markers into Scene Proposals.
    pub fn generate_proposals(performance: &CreatorPerformance) -> Vec<SceneProposal> {
        let mut proposals = Vec::new();

        for marker in &performance.markers {
            // We only care about AI Suggestions for visual assembly
            if let MarkerType::AiSuggestion = marker.marker_type {
                println!("🖼️ [ASSEMBLY] Processing AI Suggestion at {}ms: '{}'", marker.timestamp_ms, marker.description);
                
                // Mock logic: Mapping keywords in the description to actual visual assets
                let asset_id = if marker.description.contains("Rust Memory") {
                    "asset_rust_memory_diagram".to_string()
                } else {
                    "asset_generic_broll".to_string()
                };

                let proposal = SceneProposal {
                    proposal_id: Uuid::new_v4(),
                    timestamp_ms: marker.timestamp_ms,
                    visual_asset_id: asset_id,
                    layout: "PictureInPicture".to_string(), // Keeps the creator on screen!
                    description: format!("Auto-generated layout for: {}", marker.description),
                    is_approved: false, // Default to false. Creator must click "Approve".
                };

                println!("✨ [ASSEMBLY] Proposed Layout: {} with asset '{}'", proposal.layout, proposal.visual_asset_id);
                proposals.push(proposal);
            }
        }

        proposals
    }
}