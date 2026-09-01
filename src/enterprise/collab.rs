use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE OPERATION (The Intent)
// What the user (or AI Agent) wants to do.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollabOperation {
    InsertClip { track_id: Uuid, asset_id: String, start_ms: u64 },
    MoveClip { clip_id: Uuid, new_start_ms: u64 },
    DeleteClip { clip_id: Uuid },
}

// ==========================================
// 2. THE COLLABORATION EVENT (The Stamped Record)
// The operation wrapped with context for the sync engine.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollabEvent {
    pub event_id: Uuid,
    pub project_id: Uuid,
    pub actor_id: Uuid,        // Can be a User or an AI Agent!
    pub base_version: u64,     // The timeline version the actor was looking at
    pub operation: CollabOperation,
    pub timestamp_ms: u64,
}

// ==========================================
// 3. THE SYNCHRONIZATION ENGINE
// Ensures conflict-safe state resolution.
// ==========================================
pub struct CollabEngine;

impl CollabEngine {
    /// Applies an incoming event to the project. Rejects it if it causes a state conflict.
    pub fn process_event(current_project_version: u64, event: CollabEvent) -> Result<u64, String> {
        println!("🔄 [COLLAB] Processing event from Actor {}...", event.actor_id);
        
        // CONFLICT DETECTION: 
        // If the client's base version is older than the server's version, 
        // their state is stale. We reject the operation to prevent corruption.
        if event.base_version < current_project_version {
            let err = format!(
                "⚠️ [COLLAB] Conflict Detected! Actor is on v{}, but server is on v{}. Rejecting event.", 
                event.base_version, current_project_version
            );
            println!("{}", err);
            return Err(err);
        }

        // Apply the operation (In reality, this modifies the Timeline State Machine)
        match &event.operation {
            CollabOperation::InsertClip { asset_id, .. } => {
                println!("   ├─ Action: Insert Clip '{}'", asset_id);
            },
            CollabOperation::MoveClip { clip_id, new_start_ms } => {
                println!("   ├─ Action: Move Clip '{}' to {}ms", clip_id, new_start_ms);
            },
            CollabOperation::DeleteClip { clip_id } => {
                println!("   ├─ Action: Delete Clip '{}'", clip_id);
            },
        }

        let new_version = current_project_version + 1;
        println!("✅ [COLLAB] Event Applied. Project advanced to v{}", new_version);
        
        // At this point, the server would broadcast the accepted `CollabEvent` 
        // to all other connected clients so their local states update.
        Ok(new_version)
    }
}