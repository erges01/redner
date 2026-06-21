use serde::{Deserialize, Serialize};

use crate::models::timeline::TimelineDocument;

#[derive(Debug, Serialize)]
pub struct TimelineResponse {
    pub timeline: TimelineDocument,
}

#[derive(Debug, Deserialize)]
pub struct SaveTimelineRequest {
    pub timeline: TimelineDocument,
}