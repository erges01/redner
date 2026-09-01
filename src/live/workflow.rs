use crate::live::session::LiveSessionManager;
use crate::live::assembly::{LiveAssembler, SceneProposal};

// ==========================================
// THE LIVE WORKFLOW COORDINATOR
// Bridges the Live Studio to the Redner Timeline Runtime.
// ==========================================
pub struct LiveWorkflowCoordinator;

impl LiveWorkflowCoordinator {
    /// Orchestrates the entire live-to-timeline pipeline
    pub fn process_live_session(session: &mut LiveSessionManager) {
        println!("==================================================");
        println!("🔄 [WORKFLOW] Initiating Live-to-Timeline Assembly...");
        
        // 1. Run the AI Assembler to get concrete visual proposals
        let mut proposals = LiveAssembler::generate_proposals(&session.performance);
        
        if proposals.is_empty() {
            println!("ℹ️ [WORKFLOW] No AI suggestions generated for this session.");
            return;
        }

        // 2. The Human Approval Gate (Simulating UI interactions)
        println!("👤 [WORKFLOW] Presenting {} Scene Proposals to Creator UI...", proposals.len());
        for proposal in &mut proposals {
            // In the real app, this halts and waits for a WebSocket `ApproveProposal` event.
            // We simulate the creator clicking "Approve" on the toast notification.
            proposal.is_approved = true; 
            println!("✅ [WORKFLOW] Creator approved layout: {}", proposal.description);
        }
        
        // 3. Translate Approved Proposals into Timeline Commands
        println!("⚙️ [WORKFLOW] Compiling approvals into strict Editor Commands...");
        for proposal in proposals.into_iter().filter(|p| p.is_approved) {
            // This is where Phase 10 connects to Phase 2 (The Runtime Command System)!
            println!(
                "🎬 [COMMAND PIPELINE] -> INSERT ASSET '{}' AT {}ms (Layout: {})", 
                proposal.visual_asset_id, 
                proposal.timestamp_ms, 
                proposal.layout
            );
        }
        
        println!("🚀 [WORKFLOW] Live Creator Workflow Complete! Timeline is ready.");
        println!("==================================================");
    }
}