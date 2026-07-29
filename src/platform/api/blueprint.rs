use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value;

/// The public contract for generating and validating Blueprints.
#[async_trait]
pub trait BlueprintApi: Send + Sync {
    /// Calls the AI Intelligence layer to generate a Blueprint from a text prompt.
    async fn generate_from_prompt(&self, prompt: &str, persona_id: Option<Uuid>) -> Result<Value, String>;

    /// Validates a raw JSON blueprint to ensure it meets Redner's strict schema.
    async fn validate_blueprint(&self, blueprint_payload: &Value) -> Result<bool, String>;

    /// Compiles a validated Blueprint directly into the Timeline of a project.
    async fn apply_to_timeline(&self, project_id: Uuid, blueprint_payload: Value) -> Result<(), String>;
}