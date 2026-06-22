use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    dto::project_dto::{CreateProjectRequest, ProjectEditorResponse},
    error::app_error::AppError,
    models::project::Project,
};

pub async fn get_project_editor(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectEditorResponse>, AppError> {
    let response = state.project_service.get_editor_payload(project_id).await?;
    Ok(Json(response))
}

// --- NEW HANDLER ---
pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    let project = state.project_service.create_project(&payload.name).await?;
    Ok(Json(project))
}