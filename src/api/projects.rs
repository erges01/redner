use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    dto::project_dto::ProjectEditorResponse,
    error::app_error::AppError,
};

pub async fn get_project_editor(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectEditorResponse>, AppError> {
    let response = state.project_service.get_editor_payload(project_id).await?;
    Ok(Json(response))
}