use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    ai::router::ai_routes, // <-- New AI router import
    api::{
        health::health_check,
        projects::{create_project, get_project_editor},
        timelines::{get_timeline, save_timeline},
    },
    app::state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        // Added the .post() route here!
        .route(
            "/api/projects",
            post(create_project)
        )
        .route("/api/projects/:project_id/editor", get(get_project_editor))
        .route(
            "/api/projects/:project_id/timeline",
            get(get_timeline).put(save_timeline),
        )
        // Cleanly mount all AI endpoints under /api/ai
        .nest("/api/ai", ai_routes())
        .with_state(state)
}