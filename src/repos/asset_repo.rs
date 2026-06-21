use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::asset::Asset,
};

#[derive(Clone)]
pub struct AssetRepo {
    pool: PgPool,
}

impl AssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_by_project_id(&self, project_id: Uuid) -> Result<Vec<Asset>, AppError> {
        let assets = sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, project_id, name, kind, file_path, mime_type, size_bytes, created_at
            FROM assets
            WHERE project_id = $1
            ORDER BY created_at ASC
            "#
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(assets)
    }
}