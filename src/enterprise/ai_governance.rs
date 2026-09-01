use serde::{Deserialize, Serialize};

// ==========================================
// 1. ORGANIZATION AI POLICY
// Defines corporate guardrails for AI operations.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiPolicy {
    pub allowed_providers: Vec<String>,     // e.g., ["elevenlabs", "openai", "local-llama"]
    pub allowed_models: Vec<String>,        // e.g., ["gpt-4o", "claude-3-5-sonnet"]
    pub max_generation_tokens: u32,
    pub block_unverified_plugins: bool,
}

// ==========================================
// 2. THE AI GOVERNANCE ENGINE
// Audits and blocks non-compliant AI generation requests.
// ==========================================
pub struct AiGovernanceEngine;

impl AiGovernanceEngine {
    /// Validates if an incoming AI generation request complies with organization policies.
    pub fn audit_request(
        policy: &AiPolicy, 
        provider: &str, 
        model: &str, 
        requested_tokens: u32
    ) -> Result<(), String> {
        println!("🛡️ [AI GOVERNANCE] Auditing request for Provider '{}' (Model: '{}')...", provider, model);

        // 1. Check Provider Authorization
        if !policy.allowed_providers.contains(&provider.to_string()) {
            let err = format!("🛑 [AI GOVERNANCE] Blocked: Provider '{}' is not approved by organization policy.", provider);
            println!("{}", err);
            return Err(err);
        }

        // 2. Check Model Authorization
        if !policy.allowed_models.contains(&model.to_string()) {
            let err = format!("🛑 [AI GOVERNANCE] Blocked: Model '{}' is restricted under current compliance rules.", model);
            println!("{}", err);
            return Err(err);
        }

        // 3. Check Token Limit
        if requested_tokens > policy.max_generation_tokens {
            let err = format!("🛑 [AI GOVERNANCE] Blocked: Requested tokens ({}) exceed organization generation limit ({}).", 
                requested_tokens, policy.max_generation_tokens
            );
            println!("{}", err);
            return Err(err);
        }

        println!("✅ [AI GOVERNANCE] Request cleared. All enterprise compliance checks passed.");
        Ok(())
    }
}