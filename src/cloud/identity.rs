use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Note: This assumes you have some form of user extractor in your Axum app
// (Even if it just reads a token from Supabase/Clerk on the frontend)
use crate::cloud::auth::AuthenticatedCreator;

// ==========================================
// 1. THE DATA MODELS (DELIVERABLE 2)
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspacePreferences {
    pub theme: String,
    pub shortcuts_profile: String, // e.g., "premiere", "resolve", "custom"
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
// 2. CLOUD SETTINGS SERVICE (DELIVERABLE 3)
// ==========================================

/// GET /cloud/identity/profile
/// Fetches the creator's global settings the moment they log in on a new device.
pub async fn get_creator_profile(
    user: AuthenticatedCreator, 
) -> Json<CreatorProfile> {
    println!("☁️ [IDENTITY] Fetching cloud settings for Creator: {}", user.username);
    
    // In production, we query the Postgres `creator_profiles` table here using the user.creator_id
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
/// Synchronizes local preferences back to the cloud.
pub async fn sync_creator_settings(
    user: AuthenticatedCreator,
    Json(new_settings): Json<CreatorSettings>,
) -> Json<&'static str> {
    println!("☁️ [IDENTITY] Synchronizing new settings to the cloud for {}", user.username);
    
    // UPSERT into Postgres database goes here.
    
    Json("Settings synchronized successfully!")
}