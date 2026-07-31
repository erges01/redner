use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ==========================================
// 1. TIMELINE COMMENTS (DELIVERABLE 8.4)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommentTarget {
    Timestamp(f64),       // "At exactly 1m42s..."
    Clip(Uuid),           // Attached to a specific clip ID
    Track(Uuid),          // Attached to an entire track (e.g., "Mute this audio track")
    AiSuggestion(Uuid),   // Attached to an AI-generated edit proposal
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommentStatus {
    Open,
    Resolved,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineComment {
    pub comment_id: Uuid,
    pub project_id: Uuid,
    pub author_id: Uuid,
    pub target: CommentTarget,
    pub content: String,
    pub status: CommentStatus,
    pub assigned_to: Option<Uuid>, // e.g., Assigning a fix to the editor
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimelineComment {
    pub fn new(project_id: Uuid, author_id: Uuid, target: CommentTarget, content: String) -> Self {
        Self {
            comment_id: Uuid::new_v4(),
            project_id,
            author_id,
            target,
            content,
            status: CommentStatus::Open,
            assigned_to: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}