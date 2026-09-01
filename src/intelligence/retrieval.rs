use serde::{Deserialize, Serialize};
use crate::intelligence::memory::{CreativeMemoryStore, ProjectMemory, CreativeDecision};
use crate::intelligence::profile::CreatorProfile;

// ==========================================
// 1. THE RETRIEVED CONTEXT
// The exact data bundle injected into the Agent's prompt
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetrievedContext {
    pub relevant_projects: Vec<ProjectMemory>,
    pub relevant_decisions: Vec<CreativeDecision>,
    pub style_directives: Vec<String>,
}

// ==========================================
// 2. THE PERSONAL RAG ENGINE
// Retrieves past creative context based on current intent
// ==========================================
pub struct PersonalRagEngine;

impl PersonalRagEngine {
    /// Queries the creator's historical memory to find context relevant to the current task.
    /// In production, this maps to vector embeddings (e.g., pgvector). Here we use semantic heuristics.
    pub fn retrieve(
        query: &str, 
        store: &CreativeMemoryStore, 
        profile: &CreatorProfile
    ) -> RetrievedContext {
        println!("🔎 [RAG ENGINE] Searching Personal Creative Memory for query: '{}'", query);
        
        let query_lower = query.to_lowercase();
        let mut context = RetrievedContext {
            relevant_projects: Vec::new(),
            relevant_decisions: Vec::new(),
            style_directives: Vec::new(),
        };

        // 1. Retrieve Projects (Mocking a vector similarity search)
        if query_lower.contains("like my other") || query_lower.contains("usual") {
            println!("📚 [RAG ENGINE] Match found: Pulling historical projects matching audience and tone.");
            if let Some(ref active) = store.active_project {
                // In reality, this queries the DB for previous projects with similar metadata
                context.relevant_projects.push(active.clone()); 
            }
        }

        // 2. Retrieve Past Decisions (Negative constraints & guardrails)
        for decision in &store.decision_history {
            if query_lower.contains("transition") || query_lower.contains("effect") || query_lower.contains("intro") {
                // If the user's prompt touches an area where they previously rejected an AI edit
                if decision.target_component.to_lowercase().contains("transition") {
                    println!("🛡️ [RAG ENGINE] Guardrail retrieved: Creator previously rejected similar components.");
                    context.relevant_decisions.push(decision.clone());
                }
            }
        }

        // 3. Inject High-Confidence Style Directives from the Bayesian Profile
        if let Some(pacing) = profile.get_dominant_editing_style() {
            if pacing.confidence > 0.75 { // Only inject if we are mathematically sure
                context.style_directives.push(format!("Enforce pacing: {}", pacing.value));
            }
        }
        
        // Always inject global non-negotiable rules
        context.style_directives.extend(store.creator.global_rules.clone());

        println!("✅ [RAG ENGINE] Built Personalized Context Payload.");
        context
    }
}