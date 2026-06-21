use sqlx::PgPool;
use std::sync::Arc;

use crate::services::{
    project_service::ProjectService,
    timeline_service::TimelineService,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub project_service: Arc<ProjectService>,
    pub timeline_service: Arc<TimelineService>,
}