use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::project::Project,
};

#[derive(Clone)]
pub struct ProjectRepo {
    pool: PgPool,
}

impl ProjectRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, project_id: Uuid) -> Result<Option<Project>, AppError> {
        let project = sqlx::query_as::<_, Project>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM projects
            WHERE id = $1
            "#
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(project)
    }
}