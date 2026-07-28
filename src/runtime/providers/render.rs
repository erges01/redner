use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait RenderProvider: Send + Sync {
    async fn render_timeline(
        &self,
        timeline_path: &str,
        output_path: PathBuf,
    ) -> Result<(), String>;
}