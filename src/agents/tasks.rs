use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::agents::models::AgentRole;

// ==========================================
// 1. THE TASK STATUS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Reviewing,
    Completed,
    Failed(String),
}

// ==========================================
// 2. THE TASK NODE (DAG)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskNode {
    pub task_id: Uuid,
    pub description: String,
    pub required_role: AgentRole, // Who is allowed to do this?
    pub dependencies: Vec<Uuid>,  // Which tasks must finish before this one starts?
    pub status: TaskStatus,
}

// ==========================================
// 3. THE MASTER TASK GRAPH
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskGraph {
    pub graph_id: Uuid,
    pub goal: String,
    pub nodes: Vec<TaskNode>,
}