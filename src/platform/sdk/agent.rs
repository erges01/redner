use serde::{Deserialize, Serialize};
use crate::platform::api::ecosystem::PublicProject;

// ==========================================
// 1. AGENT DTOs
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentCapability {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentResult {
    pub summary: String,
    pub proposed_commands: Vec<String>, // e.g., ["INSERT_CLIP asset_123 0ms"]
}

// ==========================================
// THE AGENT SDK TRAIT
// Any 3rd-party developer building an AI Agent MUST implement this.
// ==========================================
pub trait ExternalAgent {
    /// The unique name of the third-party agent
    fn name(&self) -> &str;

    /// What tools/capabilities does this agent request?
    fn capabilities(&self) -> Vec<AgentCapability>;

    /// The core reasoning loop. 
    /// Redner passes the sanitized `PublicProject` context and the user's prompt.
    /// The Agent returns an `AgentResult` containing proposed edits.
    fn run(&self, project_context: &PublicProject, prompt: &str) -> Result<AgentResult, String>;
}