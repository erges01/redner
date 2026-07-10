// src/ai/router.rs
use axum::{routing::post, Router};
use crate::app::state::AppState; 
use super::handlers::copilot_chat;

// We return a Router configured to accept your AppState
pub fn ai_routes() -> Router<AppState> {
    Router::new()
        .route("/copilot", post(copilot_chat))
}