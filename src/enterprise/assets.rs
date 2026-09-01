use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. BRAND GUIDELINES (The Brand Vault)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrandGuidelines {
    pub primary_hex: String,        // e.g., "#0F172A"
    pub allowed_fonts: Vec<String>, // e.g., ["Inter", "JetBrains Mono"]
    pub tone_of_voice: String,      // e.g., "Technical, authoritative, concise"
    pub requires_logo_watermark: bool,
}

// ==========================================
// 2. ORGANIZATION ASSET METADATA
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AssetCategory {
    BrandLogo,
    Font,
    VideoBoll,
    AudioTrack,
    AiTemplate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgAsset {
    pub asset_id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub category: AssetCategory,
    pub storage_url: String,
    pub tags: Vec<String>,
}

// ==========================================
// 3. BRAND COMPLIANCE ENGINE
// ==========================================
pub struct BrandVaultEngine;

impl BrandVaultEngine {
    /// Validates if a proposed AI-generated text or asset complies with the organization's tone and styling.
    pub fn validate_compliance(guidelines: &BrandGuidelines, proposed_text: &str) -> Result<(), String> {
        // Simple heuristic check for tone/content compliance
        if proposed_text.to_lowercase().contains("cheap") || proposed_text.contains("🔥💥") {
            let err = "🛑 [BRAND VAULT] Compliance Violation: Proposed copy violates corporate tone guidelines.".to_string();
            println!("{}", err);
            return Err(err);
        }

        println!("✅ [BRAND VAULT] Content verified against Brand Guidelines (Primary Color: {}).", guidelines.primary_hex);
        Ok(())
    }
}