use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value;

/// The public contract for interacting with Creator Personas.
/// Plugins can use this to apply specific pacing, branding, or AI voices to a project.
#[async_trait]
pub trait CreatorApi: Send + Sync {
    /// Loads a specific Creator Persona by ID.
    async fn get_persona(&self, persona_id: Uuid) -> Result<Value, String>;

    /// Switches the active persona for a given project, automatically applying its rules.
    async fn switch_persona(&self, project_id: Uuid, persona_id: Uuid) -> Result<(), String>;
    
    /// Modifies a specific setting (e.g., pacing, primary font) on the active persona.
    async fn update_persona_setting(&self, persona_id: Uuid, key: &str, value: Value) -> Result<(), String>;
}