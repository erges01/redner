use axum::{
    routing::{get, post, put},
    Router,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

// Import every engine we built in Phase 8
use crate::cloud::identity;
use crate::cloud::presence::{self, SharedRooms};
// use crate::cloud::organizations;
// use crate::cloud::assets;

// ==========================================
// REDNER CLOUD PLATFORM API (DELIVERABLE 8.8)
// ==========================================

/// Builds the master Axum router for the entire Cloud Platform.
/// This can be deployed to Fly.io, AWS, or Render independently of the desktop app.
pub fn build_cloud_router() -> Router {
    // Initialize the in-memory multiplayer room state (Phase 8.3)
    let shared_rooms: SharedRooms = Arc::new(Mutex::new(HashMap::new()));

    // 1. Identity & Settings Routes (Phase 8.1)
    let identity_routes = Router::new()
        .route("/profile", get(identity::get_creator_profile))
        .route("/settings", put(identity::sync_creator_settings));

    // 2. Multiplayer & Sync Routes (Phase 8.2 & 8.3)
    let collab_routes = Router::new()
        .route("/:project_id/connect", get(presence::collab_ws_handler))
        .with_state(shared_rooms);

    // 3. (Placeholders for the remaining domains)
    let org_routes = Router::new(); /* e.g., POST /organizations/invite */
    let asset_routes = Router::new(); /* e.g., GET /assets/brand-kits */
    let version_routes = Router::new(); /* e.g., POST /projects/:id/commits */
    let review_routes = Router::new(); /* e.g., POST /projects/:id/comments */

    // 4. Mount everything under the /api/v1 prefix
    Router::new()
        .nest("/api/v1/identity", identity_routes)
        .nest("/api/v1/collab", collab_routes)
        .nest("/api/v1/organizations", org_routes)
        .nest("/api/v1/assets", asset_routes)
        .nest("/api/v1/versions", version_routes)
        .nest("/api/v1/reviews", review_routes)
}