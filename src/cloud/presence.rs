use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

use crate::cloud::sync::SyncOperation;

// ==========================================
// 1. THE PRESENCE MODELS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CursorPosition {
    pub track_id: Option<Uuid>,
    pub timestamp: f64, // Where their playhead/mouse is on the timeline
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPresence {
    pub creator_id: Uuid,
    pub username: String,
    pub active_tool: String, // e.g., "razor_tool", "selection_tool"
    pub cursor: Option<CursorPosition>,
}

/// The universal packet sent over the WebSocket
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", content = "data")]
pub enum CollabMessage {
    PresenceUpdate(UserPresence),
    SyncOp(SyncOperation), // 👈 This imports from Phase 8.2!
}

// ==========================================
// 2. THE MULTIPLAYER ROOM STATE
// ==========================================
pub struct ProjectRoom {
    pub project_id: Uuid,
    // A channel that broadcasts messages to everyone connected to this project
    pub tx: broadcast::Sender<String>, 
}

// Shared memory across all Axum threads holding active project rooms
pub type SharedRooms = Arc<Mutex<HashMap<Uuid, ProjectRoom>>>;

// ==========================================
// 3. THE WEBSOCKET HANDLER
// ==========================================
/// GET /cloud/projects/:project_id/collab
pub async fn collab_ws_handler(
    ws: WebSocketUpgrade,
    Path(project_id): Path<Uuid>,
    // In production, we also extract the AuthenticatedCreator here
    State(rooms): State<SharedRooms>,
) -> impl IntoResponse {
    println!("☁️ [PRESENCE] New connection request for project {}", project_id);
    
    // Upgrade the HTTP request to a WebSocket connection
    ws.on_upgrade(move |socket| handle_socket(socket, project_id, rooms))
}

async fn handle_socket(socket: WebSocket, project_id: Uuid, rooms: SharedRooms) {
    let (mut sender, mut receiver) = socket.split();

    // 1. Find or create the Project Room
    let mut rx = {
        let mut rooms_lock = rooms.lock().await;
        let room = rooms_lock.entry(project_id).or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(100);
            ProjectRoom { project_id, tx }
        });
        room.tx.subscribe()
    };

    // 2. In a real app, we spawn two Tokio tasks here:
    // Task A: Listen for messages from THIS user and broadcast them to the room's `tx`.
    // Task B: Listen to the room's `rx` channel and send those messages down to THIS user.
    
    println!("🟢 [PRESENCE] User successfully joined project room {}", project_id);
}