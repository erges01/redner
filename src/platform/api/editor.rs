use async_trait::async_trait;
use uuid::Uuid;

/// The master contract for project lifecycle management.
#[async_trait]
pub trait EditorApi: Send + Sync {
    /// Creates a new, empty Redner project and returns its UUID.
    async fn create_project(&self, name: &str) -> Result<Uuid, String>;

    /// Opens an existing project.
    async fn open_project(&self, project_id: Uuid) -> Result<(), String>;

    /// Saves the current state of a project to the database/disk.
    async fn save_project(&self, project_id: Uuid) -> Result<(), String>;

    /// Closes the project and frees up memory.
    async fn close_project(&self, project_id: Uuid) -> Result<(), String>;
}