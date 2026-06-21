use axum::{
    routing::{get, put},
    Router,
};

use crate::{
    api::{
        health::health_check,
        projects::get_project_editor,
        timelines::{get_timeline, save_timeline},
    },
    app::state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/projects/:project_id/editor", get(get_project_editor))
        .route(
            "/api/projects/:project_id/timeline",
            get(get_timeline).put(save_timeline),
        )
        .with_state(state)
}