use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait CompositionProvider: Send + Sync {
    async fn generate_timeline(
        &self,
        scene_path: &str,
        voice_path: &str,
        lipsync_path: &str,
        output_path: PathBuf,
    ) -> Result<(), String>;
}