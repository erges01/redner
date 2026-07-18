use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::runtime::jobs::models::{RuntimeJob, JobStatus};
use crate::runtime::graph::graph::RuntimeGraph;

#[derive(Clone)] // 👈 FIXED ERROR 2: Now the Dispatcher can clone it!
pub struct JobStore {
    pool: PgPool,
}

impl JobStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new job in the database
    pub async fn create_job(&self, project_id: Uuid, graph: &RuntimeGraph) -> Result<Uuid, sqlx::Error> {
        let job_id = Uuid::new_v4();
        let graph_json = serde_json::to_value(graph).unwrap();

        // 👈 FIXED ERROR 1: Changed `query!` to `query` (no bang) to bypass Neon's compile-time pooler
        sqlx::query(
            r#"
            INSERT INTO runtime_jobs (id, project_id, status, graph_state)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(job_id)
        .bind(project_id)
        .bind("pending")
        .bind(graph_json)
        .execute(&self.pool)
        .await?;

        Ok(job_id)
    }

    /// Updates the job status and the live graph state
    pub async fn update_job_state(&self, job_id: Uuid, status: &JobStatus, graph: &RuntimeGraph) -> Result<(), sqlx::Error> {
        let graph_json = serde_json::to_value(graph).unwrap();
        
        let status_str = match status {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Cancelled => "cancelled",
            JobStatus::Failed(_) => "failed",
        };

        sqlx::query(
            r#"
            UPDATE runtime_jobs 
            SET status = $1, graph_state = $2, updated_at = NOW()
            WHERE id = $3
            "#
        )
        .bind(status_str)
        .bind(graph_json)
        .bind(job_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Fetches a job so the frontend can check progress
    pub async fn get_job(&self, job_id: Uuid) -> Result<RuntimeJob, sqlx::Error> {
        let record = sqlx::query(
            r#"
            SELECT id, project_id, status, graph_state, created_at, updated_at 
            FROM runtime_jobs WHERE id = $1
            "#
        )
        .bind(job_id)
        .fetch_one(&self.pool)
        .await?;

        // Extract using sqlx::Row methods
        let graph_state_val: serde_json::Value = record.get("graph_state");
        let graph_state: RuntimeGraph = serde_json::from_value(graph_state_val).unwrap();
        
        let status_raw: String = record.get("status");
        let status = if status_raw.starts_with("failed") {
            JobStatus::Failed(status_raw)
        } else {
            match status_raw.as_str() {
                "pending" => JobStatus::Pending,
                "running" => JobStatus::Running,
                "completed" => JobStatus::Completed,
                "cancelled" => JobStatus::Cancelled,
                _ => JobStatus::Pending,
            }
        };

        Ok(RuntimeJob {
            id: record.get("id"),
            project_id: record.get("project_id"),
            status,
            graph_state,
            created_at: record.get("created_at"),
            updated_at: record.get("updated_at"),
        })
    }
}