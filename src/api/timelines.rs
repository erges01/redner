use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    app::state::AppState,
    dto::timeline_dto::{SaveTimelineRequest, TimelineResponse},
    error::app_error::AppError,
};

pub async fn get_timeline(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<TimelineResponse>, AppError> {
    let timeline = state
        .timeline_service
        .get_or_create_timeline(project_id)
        .await?;

    Ok(Json(TimelineResponse { timeline }))
}

pub async fn save_timeline(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<SaveTimelineRequest>,
) -> Result<Json<TimelineResponse>, AppError> {
    let timeline = state
        .timeline_service
        .save_timeline(project_id, payload.timeline)
        .await?;

    Ok(Json(TimelineResponse { timeline }))
}