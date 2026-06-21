use sqlx::{types::Json, PgPool};
use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::timeline::TimelineDocument,
};

#[derive(Clone)]
pub struct TimelineRepo {
    pool: PgPool,
}

impl TimelineRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Option<TimelineDocument>, AppError> {
        let row = sqlx::query_scalar::<_, Json<TimelineDocument>>(
            r#"
            SELECT timeline_json
            FROM project_timelines
            WHERE project_id = $1
            "#
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|json| json.0))
    }

    pub async fn upsert(
        &self,
        project_id: Uuid,
        timeline: &TimelineDocument,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO project_timelines (project_id, schema_version, timeline_json, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            ON CONFLICT (project_id)
            DO UPDATE SET
              schema_version = EXCLUDED.schema_version,
              timeline_json = EXCLUDED.timeline_json,
              updated_at = NOW()
            "#
        )
        .bind(project_id)
        .bind(1_i32)
        .bind(Json(timeline))
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}