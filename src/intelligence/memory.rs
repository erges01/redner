use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. CREATOR MEMORY (Global Scope)
// Long-term personal traits, defaults, and rules
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorMemory {
    pub creator_id: Uuid,
    pub preferred_video_length: String, // e.g., "short (<60s)", "long-form"
    pub preferred_tone: String,         // e.g., "technical", "conversational"
    pub caption_style: String,          // e.g., "minimal", "bold_monokai"
    pub preferred_aspect_ratio: String, // e.g., "9:16", "16:9"
    pub global_rules: Vec<String>,      // e.g., ["No generic stock music", "Code snippets must be visible"]
}

// ==========================================
// 2. PROJECT MEMORY (Local Project Scope)
// Explicit context for the active timeline
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMemory {
    pub project_id: Uuid,
    pub creator_id: Uuid,
    pub creative_direction: String,     // e.g., "Rust ownership deep dive"
    pub target_audience: String,        // e.g., "Intermediate systems engineers"
    pub visual_theme: String,           // e.g., "Dark minimal with terminal captures"
    pub active_constraints: Vec<String>,// e.g., ["Must finish under 90s", "Keep hook under 5s"]
}

// ==========================================
// 3. DECISION MEMORY (Feedback Ledger)
// Records past creator approvals & rejections
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DecisionAction {
    Approved,
    Rejected,
    Modified { original: String, revised: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeDecision {
    pub decision_id: Uuid,
    pub project_id: Uuid,
    pub target_component: String,      // e.g., "Transition: 3D Whip Pan"
    pub action: DecisionAction,
    pub creator_feedback: Option<String>, // e.g., "Disliked flashiness, prefer clean cut"
    pub timestamp_ms: u64,
}

// ==========================================
// 4. STYLE MEMORY (Statistical Baseline)
// Concrete metrics extracted from timeline history
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleMemory {
    pub creator_id: Uuid,
    pub average_shot_duration_ms: u32,  // e.g., 2800 (2.8 seconds)
    pub typical_hook_duration_ms: u32,  // e.g., 6500 (6.5 seconds)
    pub b_roll_frequency_per_min: f32,  // e.g., 4.2
    pub transition_density: String,     // e.g., "minimal", "high"
}

// ==========================================
// 5. UNIFIED CREATIVE MEMORY STORE
// Aggregated memory bundle injected into Agent Context
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeMemoryStore {
    pub creator: CreatorMemory,
    pub active_project: Option<ProjectMemory>,
    pub decision_history: Vec<CreativeDecision>,
    pub style: StyleMemory,
}

impl CreativeMemoryStore {
    /// Loads a baseline initialized store with default creator traits
    pub fn mock_initial_state(creator_id: Uuid, project_id: Uuid) -> Self {
        Self {
            creator: CreatorMemory {
                creator_id,
                preferred_video_length: "short (<60s)".to_string(),
                preferred_tone: "developer-focused, analytical".to_string(),
                caption_style: "minimal_monokai".to_string(),
                preferred_aspect_ratio: "9:16".to_string(),
                global_rules: vec![
                    "Cut all dead air exceeding 1.2s".to_string(),
                    "Highlight terminal commands in green".to_string(),
                ],
            },
            active_project: Some(ProjectMemory {
                project_id,
                creator_id,
                creative_direction: "Demystifying Rust Borrow Checker".to_string(),
                target_audience: "Backend engineers transitioning to systems programming".to_string(),
                visual_theme: "Dark IDE / Terminal focused".to_string(),
                active_constraints: vec!["Target run-time: 58 seconds".to_string()],
            }),
            decision_history: vec![
                CreativeDecision {
                    decision_id: Uuid::new_v4(),
                    project_id,
                    target_component: "Transition: Flash Zoom".to_string(),
                    action: DecisionAction::Rejected,
                    creator_feedback: Some("Distracting. Use hard cuts for code changes.".to_string()),
                    timestamp_ms: 1720000000000,
                },
            ],
            style: StyleMemory {
                creator_id,
                average_shot_duration_ms: 2400,
                typical_hook_duration_ms: 5000,
                b_roll_frequency_per_min: 5.0,
                transition_density: "minimal".to_string(),
            },
        }
    }

    /// Logs an explicit human decision to prevent repeating rejected patterns
    pub fn log_decision(&mut self, decision: CreativeDecision) {
        println!(
            "🧠 [CREATIVE MEMORY] Recorded Decision: {:?} on '{}'",
            decision.action, decision.target_component
        );
        self.decision_history.push(decision);
    }
}