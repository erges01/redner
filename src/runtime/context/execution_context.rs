use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub project_id: Uuid,
    pub session_id: String,
    
    /// A temporary directory where nodes can save intermediate files (e.g., downloaded audio)
    pub workspace_dir: PathBuf,
    
    // 🚧 Future Additions:
    // pub db_pool: sqlx::PgPool,
    // pub http_client: reqwest::Client,
    // pub s3_client: SomeS3Client,
}

impl ExecutionContext {
    pub fn new(project_id: Uuid, base_temp_dir: PathBuf) -> Self {
        let session_id = uuid::Uuid::new_v4().to_string();
        let workspace_dir = base_temp_dir.join(project_id.to_string()).join(&session_id);
        
        Self {
            project_id,
            session_id,
            workspace_dir,
        }
    }
}