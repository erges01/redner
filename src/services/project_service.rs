use uuid::Uuid;

use crate::{
    dto::project_dto::ProjectEditorResponse,
    error::app_error::AppError,
    models::project::Project,
    repos::{asset_repo::AssetRepo, project_repo::ProjectRepo},
    services::timeline_service::TimelineService,
};

#[derive(Clone)]
pub struct ProjectService {
    project_repo: ProjectRepo,
    asset_repo: AssetRepo,
    timeline_service: TimelineService,
}

impl ProjectService {
    pub fn new(
        project_repo: ProjectRepo,
        asset_repo: AssetRepo,
        timeline_service: TimelineService,
    ) -> Self {
        Self {
            project_repo,
            asset_repo,
            timeline_service,
        }
    }

    pub async fn get_editor_payload(
        &self,
        project_id: Uuid,
    ) -> Result<ProjectEditorResponse, AppError> {
        let project = self
            .project_repo
            .get_by_id(project_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("project {} not found", project_id)))?;

        let assets = self.asset_repo.list_by_project_id(project_id).await?;
        let timeline = self.timeline_service.get_or_create_timeline(project_id).await?;

        Ok(ProjectEditorResponse {
            project,
            assets,
            timeline,
        })
    }

    // --- NEW METHOD ---
    pub async fn create_project(&self, name: &str) -> Result<Project, AppError> {
        self.project_repo.create(name).await
    }
}