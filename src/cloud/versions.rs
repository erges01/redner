use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ==========================================
// 1. THE COMMIT AUTHOR
// AI actions must be completely transparent
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommitAuthor {
    Creator(Uuid),
    AiAgent { persona_id: Uuid, model: String }, // e.g., "Director Persona", "Claude 3.5 Sonnet"
}

// ==========================================
// 2. THE VERSION COMMIT (DELIVERABLE 8.5)
// Git for Video Editing
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionCommit {
    pub commit_id: Uuid,
    pub project_id: Uuid,
    
    // If a creator experiments with a different ending, they can branch off an older commit!
    pub parent_commit_id: Option<Uuid>, 
    
    pub author: CommitAuthor,
    pub message: String, // e.g., "Tightened the pacing in the intro"
    
    // THIS IS THE MAGIC. It points back to the Sync Engine (Phase 8.2)
    // To load this version, the backend just replays ops from 0 up to this number.
    pub latest_sync_sequence: i64, 
    
    pub created_at: DateTime<Utc>,
}

impl VersionCommit {
    /// Creates a standard human checkpoint
    pub fn new_human_commit(
        project_id: Uuid, 
        creator_id: Uuid, 
        parent: Option<Uuid>,
        message: String, 
        latest_sync: i64
    ) -> Self {
        Self {
            commit_id: Uuid::new_v4(),
            project_id,
            parent_commit_id: parent,
            author: CommitAuthor::Creator(creator_id),
            message,
            latest_sync_sequence: latest_sync,
            created_at: Utc::now(),
        }
    }

    /// Creates an AI checkpoint (No hidden mutations allowed!)
    pub fn new_ai_commit(
        project_id: Uuid, 
        persona_id: Uuid, 
        model: String,
        parent: Option<Uuid>,
        message: String, 
        latest_sync: i64
    ) -> Self {
        Self {
            commit_id: Uuid::new_v4(),
            project_id,
            parent_commit_id: parent,
            author: CommitAuthor::AiAgent { persona_id, model },
            message,
            latest_sync_sequence: latest_sync,
            created_at: Utc::now(),
        }
    }
}