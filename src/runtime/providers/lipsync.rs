use std::path::PathBuf;
use async_trait::async_trait;

#[async_trait]
pub trait LipSyncProvider: Send + Sync {
    // Takes the generated audio path, processes it, and writes the mouth shapes to the output JSON path
    async fn generate_visemes(
        &self, 
        audio_path: PathBuf, 
        output_path: PathBuf
    ) -> Result<(), String>;
}