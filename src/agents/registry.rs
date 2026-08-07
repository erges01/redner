use std::collections::HashMap;
use uuid::Uuid;
use crate::agents::models::{AgentDefinition, AgentRole, AgentStatus};

// ==========================================
// THE AGENT REGISTRY
// The central hub for all Redner AI Agents
// ==========================================
#[derive(Debug, Clone)]
pub struct AgentRegistry {
    // Maps the AgentRole to its Definition
    agents: HashMap<String, AgentDefinition>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            agents: HashMap::new(),
        };
        registry.initialize_default_team();
        registry
    }

    /// Pre-loads Redner's core autonomous creative team
    fn initialize_default_team(&mut self) {
        // 1. The Creative Director (Orchestrator)
        self.register(AgentDefinition {
            agent_id: Uuid::new_v4(),
            name: "Director_Alpha".to_string(),
            role: AgentRole::CreativeDirector,
            system_prompt: "You are the Creative Director. Your job is to decompose the user's goal into a Task Graph and assign tasks to specialized agents. You do not edit media directly.".to_string(),
            allowed_tools: vec!["create_task".to_string(), "assign_task".to_string(), "request_approval".to_string()],
            status: AgentStatus::Idle,
        });

        // 2. The Script Specialist
        self.register(AgentDefinition {
            agent_id: Uuid::new_v4(),
            name: "Scripter_V1".to_string(),
            role: AgentRole::ScriptSpecialist,
            system_prompt: "You are the Script Specialist. Write engaging hooks, structure narrative pacing, and define CTAs. Ensure the tone matches the Creator's AI Memory.".to_string(),
            allowed_tools: vec!["generate_script".to_string(), "read_creator_memory".to_string()],
            status: AgentStatus::Idle,
        });

        // 3. The Editor Agent
        self.register(AgentDefinition {
            agent_id: Uuid::new_v4(),
            name: "Editor_V1".to_string(),
            role: AgentRole::Editor,
            system_prompt: "You are the Editor Agent. You execute precise timeline mutations. You place clips, trim silence, and mix audio.".to_string(),
            allowed_tools: vec!["split_clip".to_string(), "move_clip".to_string(), "apply_transition".to_string()],
            status: AgentStatus::Idle,
        });

        // 4. The Review Agent (QA)
        self.register(AgentDefinition {
            agent_id: Uuid::new_v4(),
            name: "Reviewer_QA".to_string(),
            role: AgentRole::Reviewer,
            system_prompt: "You are the Review Agent. You critique pacing, visual flow, and coherence. If a problem is detected, send a revision request back to the Editor or Director.".to_string(),
            allowed_tools: vec!["leave_timeline_comment".to_string(), "request_revision".to_string(), "approve_timeline".to_string()],
            status: AgentStatus::Idle,
        });
    }

    /// Registers a new agent into the system
    pub fn register(&mut self, agent: AgentDefinition) {
        let role_key = format!("{:?}", agent.role);
        self.agents.insert(role_key, agent);
    }

    /// Fetches an agent by its role
    pub fn get_by_role(&self, role: &AgentRole) -> Option<AgentDefinition> {
        let role_key = format!("{:?}", role);
        self.agents.get(&role_key).cloned()
    }
}