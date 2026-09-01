use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. PUBLIC ECOSYSTEM DTOs
// The strict data contract for third-party developers.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicProject {
    pub id: Uuid,
    pub name: String,
    pub resolution: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicClip {
    pub id: Uuid,
    pub asset_id: String,
    pub start_ms: u64,
}

// ==========================================
// 2. THE REDNER ECOSYSTEM CLIENT
// The single gateway exposed to external SDKs and Plugins.
// ==========================================
pub struct RednerEcosystemClient;

impl RednerEcosystemClient {
    pub fn get_active_project(project_id: Uuid) -> Result<PublicProject, String> {
        println!("🌐 [ECOSYSTEM API] Third-party requested Project {}", project_id);
        Ok(PublicProject {
            id: project_id,
            name: "Creator Project".to_string(),
            resolution: "1920x1080".to_string(),
        })
    }

    pub fn insert_clip(asset_id: &str, start_ms: u64) -> Result<PublicClip, String> {
        println!("🌐 [ECOSYSTEM API] Third-party inserting Asset '{}' at {}ms", asset_id, start_ms);
        Ok(PublicClip {
            id: Uuid::new_v4(),
            asset_id: asset_id.to_string(),
            start_ms,
        })
    }
}