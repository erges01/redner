use serde::{Deserialize, Serialize};

use crate::models::{
    asset::Asset,
    project::Project,
    timeline::TimelineDocument,
};

#[derive(Debug, Serialize)]
pub struct ProjectEditorResponse {
    pub project: Project,
    pub assets: Vec<Asset>,
    pub timeline: TimelineDocument,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}