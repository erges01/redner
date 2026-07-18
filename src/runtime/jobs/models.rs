use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::runtime::graph::graph::RuntimeGraph;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeJob {
    pub id: Uuid,
    pub project_id: Uuid,
    pub status: JobStatus,
    pub graph_state: RuntimeGraph, // Rust will auto-convert this to/from JSONB!
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}