use serde::{Deserialize, Serialize};
use crate::platform::plugin::security::PluginManifest;

// ==========================================
// 1. TRUST LEVELS
// Determines the strictness of the WASM sandbox
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TrustLevel {
    Official,         // Built by the Redner Core Team (Full Access)
    VerifiedPartner,  // Audited enterprise companies e.g., ElevenLabs (High Access)
    Community,        // Standard indie developers (Strictly Sandboxed)
    Untrusted,        // Failed signature check or globally revoked (Blocked)
}

// ==========================================
// 2. THE GOVERNANCE POLICY
// The active security state of a specific package
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GovernancePolicy {
    pub plugin_id: String,
    pub trust_level: TrustLevel,
    pub is_revoked: bool,
    pub revocation_reason: Option<String>,
}

// ==========================================
// 3. THE GOVERNANCE ENGINE
// The global oversight authority that audits all loaded extensions
// ==========================================
pub struct GovernanceEngine;

impl GovernanceEngine {
    /// Evaluates a plugin against the global revocation list and assigns a Trust Level.
    /// This runs before the PluginRuntime is allowed to execute the WASM binary.
    pub fn evaluate_plugin(manifest: &PluginManifest) -> GovernancePolicy {
        println!("🛡️ [GOVERNANCE] Auditing plugin payload: {} (v{})", manifest.plugin_id, manifest.version);
        
        // 1. Check Global Kill Switch (Mocking a database query)
        let is_revoked = manifest.plugin_id == "com.sketchydev.asset-stealer";
        
        // 2. Assign Trust Level based on publisher verification
        let trust_level = if is_revoked {
            TrustLevel::Untrusted
        } else if manifest.publisher == "Redner Inc" {
            TrustLevel::Official
        } else if manifest.publisher == "ElevenLabs" {
            TrustLevel::VerifiedPartner
        } else {
            TrustLevel::Community // Default sandbox for indie devs
        };

        if is_revoked {
            println!("🚨 [GOVERNANCE] ALERT: Execution BLOCKED. Plugin '{}' is on the global revocation list!", manifest.plugin_id);
        } else {
            println!("✅ [GOVERNANCE] Audit cleared. Trust Level: {:?}", trust_level);
        }

        GovernancePolicy {
            plugin_id: manifest.plugin_id.clone(),
            trust_level,
            is_revoked,
            revocation_reason: if is_revoked { 
                Some("Violation of API TOS: Attempted unauthorized network exfiltration.".to_string()) 
            } else { 
                None 
            },
        }
    }
}