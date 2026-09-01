use crate::intelligence::retrieval::RetrievedContext;
use crate::intelligence::project::ProjectIntent;

// ==========================================
// THE PROMPT AUGMENTER
// Fuses raw agent instructions with Creator Intelligence
// ==========================================
pub struct PromptAugmenter;

impl PromptAugmenter {
    /// Takes a generic agent task and dynamically rewrites the prompt to force 
    /// the AI to obey the creator's historical style and guardrails.
    pub fn augment_task_prompt(
        base_instruction: &str,
        context: &RetrievedContext,
        intent: &ProjectIntent,
    ) -> String {
        println!("🧬 [PERSONALIZATION] Augmenting agent prompt with Creator DNA...");

        let mut personalized_prompt = format!("### TASK INSTRUCTION ###\n{}\n\n", base_instruction);

        // 1. Inject Project Intent
        personalized_prompt.push_str("### PROJECT CONTEXT ###\n");
        personalized_prompt.push_str(&format!("Primary Goal: {}\n", intent.primary_goal));
        personalized_prompt.push_str(&format!("Emotional Tone: {}\n", intent.emotional_tone));
        personalized_prompt.push_str(&format!("Complexity: {}\n\n", intent.complexity_level));

        // 2. Inject Bayesian Style Directives (from Phase 11.2 & 11.3)
        if !context.style_directives.is_empty() {
            personalized_prompt.push_str("### CREATOR STYLE DIRECTIVES (MANDATORY) ###\n");
            for directive in &context.style_directives {
                personalized_prompt.push_str(&format!("- {}\n", directive));
            }
            personalized_prompt.push_str("\n");
        }

        // 3. Inject Historical Guardrails (from Phase 11.1 & 11.5)
        if !context.relevant_decisions.is_empty() {
            personalized_prompt.push_str("### GUARDRAILS & PAST REJECTIONS ###\n");
            for decision in &context.relevant_decisions {
                if let Some(feedback) = &decision.creator_feedback {
                    personalized_prompt.push_str(&format!(
                        "- DO NOT USE '{}'. (Creator previously stated: \"{}\")\n", 
                        decision.target_component, feedback
                    ));
                }
            }
            personalized_prompt.push_str("\n");
        }

        println!("✅ [PERSONALIZATION] System Prompt successfully overridden.");
        
        personalized_prompt
    }
}