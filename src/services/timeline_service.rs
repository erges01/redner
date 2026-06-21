use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::timeline::TimelineDocument,
    repos::{project_repo::ProjectRepo, timeline_repo::TimelineRepo},
};

#[derive(Clone)]
pub struct TimelineService {
    project_repo: ProjectRepo,
    timeline_repo: TimelineRepo,
}

impl TimelineService {
    pub fn new(project_repo: ProjectRepo, timeline_repo: TimelineRepo) -> Self {
        Self {
            project_repo,
            timeline_repo,
        }
    }

    pub async fn get_or_create_timeline(
        &self,
        project_id: Uuid,
    ) -> Result<TimelineDocument, AppError> {
        let project = self.project_repo.get_by_id(project_id).await?;
        if project.is_none() {
            return Err(AppError::NotFound(format!("project {} not found", project_id)));
        }

        if let Some(timeline) = self.timeline_repo.get_by_project_id(project_id).await? {
            return Ok(timeline);
        }

        let timeline = Self::default_timeline(project_id);
        self.timeline_repo.upsert(project_id, &timeline).await?;
        Ok(timeline)
    }

    pub async fn save_timeline(
        &self,
        project_id: Uuid,
        mut timeline: TimelineDocument,
    ) -> Result<TimelineDocument, AppError> {
        let project = self.project_repo.get_by_id(project_id).await?;
        if project.is_none() {
            return Err(AppError::NotFound(format!("project {} not found", project_id)));
        }

        if timeline.project_id != project_id {
            timeline.project_id = project_id;
        }

        self.validate_timeline(&timeline)?;
        self.timeline_repo.upsert(project_id, &timeline).await?;

        Ok(timeline)
    }

    fn validate_timeline(&self, timeline: &TimelineDocument) -> Result<(), AppError> {
        if timeline.fps == 0 {
            return Err(AppError::BadRequest("timeline fps must be greater than 0".into()));
        }

        for track in &timeline.tracks {
            for clip in &track.clips {
                if clip.duration_ms == 0 {
                    return Err(AppError::BadRequest(format!(
                        "clip {} has invalid duration",
                        clip.id
                    )));
                }
            }
        }

        Ok(())
    }

    fn default_timeline(project_id: Uuid) -> TimelineDocument {
        TimelineDocument {
            id: Uuid::new_v4().to_string(),
            project_id,
            fps: 30,
            duration_ms: 60_000,
            zoom: 1.0,
            playhead_ms: 0,
            tracks: vec![],
        }
    }
}