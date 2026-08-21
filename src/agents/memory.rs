use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. CREATOR MEMORY
// Who the agent is working for. 
// Completely transparent and editable by the user.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorMemory {
    pub preferred_pacing: String, // e.g., "fast-paced, jump cuts, zero dead air"
    pub tone: String,             // e.g., "highly technical, educational, direct"
    pub visual_style: String,     // e.g., "dark mode, minimalist typography"
    pub brand_rules: Vec<String>, // e.g., ["Never use generic stock music", "Always caption in uppercase"]
}

// ==========================================
// 2. PROJECT MEMORY
// What happened previously in this specific project.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMemory {
    pub current_creative_direction: String,
    pub approved_concepts: Vec<String>,
    pub rejected_concepts: Vec<String>, // "User hated the neon intro, do not try it again."
}

// ==========================================
// 3. TASK MEMORY
// For the execution loop (e.g., retrying after a failure)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskMemory {
    pub attempts: u32,
    pub last_failure_reason: Option<String>,
}

// ==========================================
// 4. THE MASTER AGENT MEMORY PAYLOAD
// This gets injected into the `AgentContext` during execution.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentMemoryPayload {
    pub memory_id: Uuid,
    pub creator_id: Uuid,
    pub project_id: Uuid,
    pub creator_context: CreatorMemory,
    pub project_context: ProjectMemory,
}

impl AgentMemoryPayload {
    /// Simulates loading the creator's explicit memory from Postgres
    pub fn load_mock_memory(creator_id: Uuid, project_id: Uuid) -> Self {
        Self {
            memory_id: Uuid::new_v4(),
            creator_id,
            project_id,
            creator_context: CreatorMemory {
                preferred_pacing: "snappy, zero silence".to_string(),
                tone: "developer-focused, analytical".to_string(),
                visual_style: "Monokai theme colors, code snippets on screen".to_string(),
                brand_rules: vec!["No cheesy transitions".to_string(), "Keep intro under 3 seconds".to_string()],
            },
            project_context: ProjectMemory {
                current_creative_direction: "Rust backend tutorials".to_string(),
                approved_concepts: vec!["Show real Axum code".to_string()],
                rejected_concepts: vec!["Don't use the whiteboard animation".to_string()],
            },
        }
    }
}