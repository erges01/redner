use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum SyncAction {
    AddTrack { name: String },
    RemoveTrack { track_id: Uuid },
    AddClip { track_id: Uuid, asset_id: Uuid, start_time: f64 },
    MoveClip { clip_id: Uuid, new_start_time: f64 },
    SplitClip { clip_id: Uuid, split_time: f64 },
    DeleteClip { clip_id: Uuid },
    UpdateAiPrompt { clip_id: Uuid, new_prompt: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncOperation {
    pub operation_id: Uuid,
    pub project_id: Uuid,
    pub creator_id: Uuid,
    pub action: SyncAction,
    pub created_at: DateTime<Utc>,
    pub version_sequence: i64,
}

impl SyncOperation {
    pub fn new(project_id: Uuid, creator_id: Uuid, action: SyncAction, version_sequence: i64) -> Self {
        Self {
            operation_id: Uuid::new_v4(),
            project_id,
            creator_id,
            action,
            created_at: Utc::now(),
            version_sequence,
        }
    }
}