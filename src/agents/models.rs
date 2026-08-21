use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// ==========================================
// 1. THE AGENT STATE MACHINE
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Planning,
    Executing,
    WaitingForHuman,// E.g., waiting for human approval or another agent
    Reviewing,
    Completed,
    Error(String),
}

// ==========================================
// 2. THE SPECIALIST ROLES
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AgentRole {
    CreativeDirector,
    ScriptSpecialist,
    VisualSpecialist,
    VoiceSpecialist,
    PerformanceSpecialist,
    Editor,
    Reviewer,
}
// ==========================================
// 3. THE AGENT DEFINITION
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentDefinition {
    pub agent_id: Uuid,
    pub name: String,
    pub role: AgentRole,
    pub system_prompt: String,
    pub allowed_tools: Vec<String>, // Hooks directly into Phase 7 Platform SDK!
    pub status: AgentStatus,
}

// ==========================================
// 4. THE AGENT CONTEXT
// What the agent knows right now
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentContext {
    pub project_id: Uuid,
    pub creator_id: Uuid,
    pub active_task_id: Option<Uuid>,
    pub memory_keys: HashMap<String, String>, // Injects the Phase 8.6 AI Memory here
}