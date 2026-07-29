use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value;

/// The public contract for triggering and monitoring generation jobs.
/// This wraps our internal GraphDispatcher and JobStore.
#[async_trait]
pub trait RuntimeApi: Send + Sync {
    /// Compiles a blueprint into a graph and starts execution. Returns the Job ID.
    async fn run_graph(&self, project_id: Uuid, blueprint_payload: Value) -> Result<Uuid, String>;

    /// Attempts to safely halt an ongoing graph execution.
    async fn cancel_job(&self, job_id: Uuid) -> Result<(), String>;

    /// Fetches the current execution status (Running, Completed, Failed) of a job.
    async fn get_job_status(&self, job_id: Uuid) -> Result<String, String>;
}