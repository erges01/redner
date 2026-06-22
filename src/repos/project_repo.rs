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

    pub async fn create(&self, name: &str) -> Result<Project, AppError> {
        let new_id = Uuid::new_v4();
        
        let project = sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (id, name, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            RETURNING id, name, description, created_at, updated_at
            "#
        )
        .bind(new_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }
}