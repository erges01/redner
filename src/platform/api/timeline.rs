use async_trait::async_trait;
use uuid::Uuid;

/// The public contract for interacting with the Redner Timeline.
/// Plugins, Agents, and Workflows MUST use this API to modify sequences.
#[async_trait]
pub trait TimelineApi: Send + Sync {
    /// Adds a new track (e.g., Video, Audio, Captions) to the project.
    async fn add_track(&self, project_id: Uuid, name: &str) -> Result<Uuid, String>;

    /// Removes a track and all its nested clips.
    async fn remove_track(&self, project_id: Uuid, track_id: Uuid) -> Result<(), String>;

    /// Moves a clip to a new start time on the timeline.
    async fn move_clip(&self, project_id: Uuid, clip_id: Uuid, new_start_time: f64) -> Result<(), String>;

    /// Splits a single clip into two distinct clips at the given timestamp.
    /// Returns the UUIDs of the two new clips.
    async fn split_clip(&self, project_id: Uuid, clip_id: Uuid, split_time: f64) -> Result<(Uuid, Uuid), String>;
    
    /// Deletes a clip entirely from the timeline.
    async fn delete_clip(&self, project_id: Uuid, clip_id: Uuid) -> Result<(), String>;
}