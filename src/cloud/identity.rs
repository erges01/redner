use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::convert::Infallible;

// ==========================================
// 1. THE MOCK AUTH EXTRACTOR 
// (Bypasses auth until Next.js is built)
// ==========================================
#[derive(Debug, Clone)]
pub struct AuthenticatedCreator {
    pub creator_id: Uuid,
    pub username: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthenticatedCreator {
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Automatically acts as if you are logged in
        Ok(AuthenticatedCreator {
            creator_id: Uuid::new_v4(),
            username: "Adesope".to_string(), 
        })
    }
}

// ==========================================
// 2. THE DATA MODELS
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspacePreferences {
    pub theme: String,
    pub shortcuts_profile: String, 
    pub auto_save_interval_secs: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorSettings {
    pub workspace: WorkspacePreferences,
    pub default_persona_id: Option<Uuid>,
    pub ai_temperature_preference: f32,
    pub installed_plugins: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatorProfile {
    pub creator_id: Uuid,
    pub display_name: String,
    pub bio: String,
    pub settings: CreatorSettings,
}

// ==========================================
// 3. CLOUD SETTINGS SERVICE
// ==========================================

/// GET /cloud/identity/profile
pub async fn get_creator_profile(
    user: AuthenticatedCreator, 
) -> Json<CreatorProfile> {
    println!("☁️ [IDENTITY] Fetching cloud settings for Creator: {}", user.username);
    
    let profile = CreatorProfile {
        creator_id: user.creator_id,
        display_name: user.username.clone(),
        bio: "Redner OS Architect".to_string(),
        settings: CreatorSettings {
            workspace: WorkspacePreferences {
                theme: "dark_mode".to_string(),
                shortcuts_profile: "redner_default".to_string(),
                auto_save_interval_secs: 300,
            },
            default_persona_id: None,
            ai_temperature_preference: 0.7,
            installed_plugins: vec!["auto-cutter".to_string(), "youtube-export-pack".to_string()],
        }
    };

    Json(profile)
}

/// PUT /cloud/identity/settings
pub async fn sync_creator_settings(
    user: AuthenticatedCreator,
    Json(_new_settings): Json<CreatorSettings>,
) -> Json<&'static str> {
    println!("☁️ [IDENTITY] Synchronizing new settings to the cloud for {}", user.username);
    Json("Settings synchronized successfully!")
}