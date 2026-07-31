use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// ==========================================
// 1. THE SHARED CLOUD ASSET
// Reusable across all projects and teams
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetType {
    VoiceProfile(String), // e.g., ElevenLabs Voice ID
    DigitalTwin,          // Video/Avatar model data
    BrandKit,             // Fonts, Hex codes, Logos
    MotionGraphic,        // Reusable .mogrt or WebGL templates
    BlueprintTemplate,    // Ties into Phase 7.6!
    PromptTemplate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudAsset {
    pub asset_id: Uuid,
    pub owner_id: Uuid, // Creator ID or Organization ID
    pub asset_type: AssetType,
    pub name: String,
    pub storage_url: Option<String>, // e.g., AWS S3 or Cloudflare R2 URL
    pub is_public: bool,             // Can be shared to the Marketplace (Phase 7.8)
    pub created_at: DateTime<Utc>,
}

// ==========================================
// 2. EXPLICIT AI MEMORY
// The AI doesn't guess your style. It reads this.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiMemory {
    pub memory_id: Uuid,
    pub creator_id: Uuid,

    // Core Directives
    pub preferred_pacing: String,  // e.g., "Aggressive, cut all dead air, YouTube Shorts style"
    pub preferred_framing: String, // e.g., "Keep subject dead center, dynamic zoom on emphasis"
    pub caption_style: String,     // e.g., "Bold yellow, uppercase, 1 word per screen"
    
    // Custom explicit rules set by the creator
    // e.g., {"b-roll": "Only use cinematic minimal tech footage"}
    pub custom_rules: HashMap<String, String>, 
    
    pub updated_at: DateTime<Utc>,
}

impl AiMemory {
    /// Helper to compile the memory into a system prompt injection for the AI Agent
    pub fn compile_system_prompt(&self) -> String {
        let mut prompt = format!(
            "You are assisting a creator. Strictly adhere to these preferences:\n\
            - Pacing: {}\n\
            - Framing: {}\n\
            - Captions: {}\n",
            self.preferred_pacing, self.preferred_framing, self.caption_style
        );

        if !self.custom_rules.is_empty() {
            prompt.push_str("- Additional Rules:\n");
            for (key, val) in &self.custom_rules {
                prompt.push_str(&format!("  * {}: {}\n", key, val));
            }
        }

        prompt
    }
}