use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait SceneProvider: Send + Sync {
    async fn generate_scene(&self, prompt: &str, output_path: PathBuf) -> Result<(), String>;
}