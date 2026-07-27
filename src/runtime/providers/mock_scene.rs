use std::path::PathBuf;
use async_trait::async_trait;
use tokio::fs;
use crate::runtime::providers::scene::SceneProvider;

pub struct MockSceneProvider;

impl MockSceneProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SceneProvider for MockSceneProvider {
    async fn generate_scene(&self, prompt: &str, output_path: PathBuf) -> Result<(), String> {
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }

        // Simulate rendering a visual scene asset based on the prompt
        let dummy_content = format!("{{\"type\": \"scene\", \"prompt\": \"{}\", \"status\": \"rendered\"}}", prompt);
        fs::write(&output_path, dummy_content).await.map_err(|e| e.to_string())?;

        println!("   ✅ [MockSceneProvider] Scene asset rendered to {:?}", output_path);
        Ok(())
    }
}