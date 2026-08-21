use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE AUTONOMY LEVELS
// How much leash does the AI have?
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AutonomyLevel {
    Manual,         // AI only suggests. Human executes.
    Assisted,       // AI prepares the edits, human must approve before they hit the timeline.
    Autonomous,     // AI edits the timeline freely, but must ask before Exporting/Publishing.
    FullProduction, // AI does everything end-to-end (Danger Zone).
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalRequest {
    pub request_id: Uuid,
    pub task_id: Uuid,
    pub summary: String,
    pub required_level: AutonomyLevel,
}

// ==========================================
// 2. THE HUMAN GATEKEEPER
// ==========================================
pub struct HumanGatekeeper;

impl HumanGatekeeper {
    /// Pauses the agent execution loop and asks the human for permission.
    /// In the final app, this sends a WebSocket event to the React frontend
    /// and suspends the Rust thread until the user clicks "Approve".
    pub fn request_approval(level: &AutonomyLevel, task_summary: &str) -> bool {
        println!("--------------------------------------------------");
        println!("✋ [GATEKEEPER] HALTING AI EXECUTION.");
        println!("✋ [GATEKEEPER] Action requires human review: '{}'", task_summary);
        println!("✋ [GATEKEEPER] Current Creator Autonomy Level: {:?}", level);
        
        if level == &AutonomyLevel::FullProduction {
            println!("🤖 [GATEKEEPER] Full Production mode is active. Auto-approving bypass...");
            return true;
        }

        // Mocking the WebSocket suspension
        println!("👤 [GATEKEEPER] Sending Approval UI Card to Frontend via WebSocket...");
        println!("👤 [GATEKEEPER] Waiting for Creator to click 'Approve'...");
        
        // Simulating the user clicking "Approve" 5 seconds later
        println!("✅ [GATEKEEPER] Human clicked APPROVE. Unlocking AI thread.");
        true
    }
}