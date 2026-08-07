use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

// ==========================================
// 1. THE EVENT PAYLOADS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum AgentEventPayload {
    AgentStarted,
    AgentThinking { thought: String },
    ToolCalled { tool_name: String, args: Value },
    ToolCompleted { tool_name: String, result: Value },
    AgentWaiting { reason: String },
    AgentCompleted { summary: String },
    AgentFailed { error: String },
}

// ==========================================
// 2. THE EVENT ENVELOPE
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentEvent {
    pub event_id: Uuid,
    pub agent_id: Uuid,
    pub project_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: AgentEventPayload,
}

impl AgentEvent {
    pub fn new(agent_id: Uuid, project_id: Uuid, payload: AgentEventPayload) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            agent_id,
            project_id,
            timestamp: Utc::now(),
            payload,
        }
    }
}