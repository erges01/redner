use crate::platform::plugin::security::PluginManifest;
use crate::platform::api::ecosystem::RednerEcosystemClient;

// ==========================================
// THE EXTENSION SDK TRAIT
// Any 3rd-party developer building a standard plugin MUST implement this.
// ==========================================
pub trait RednerExtension {
    /// 1. Identity: The plugin must declare who it is and what it needs.
    fn manifest(&self) -> PluginManifest;

    /// 2. Lifecycle: Fired exactly once when the creator clicks "Install".
    fn on_install(&self) -> Result<(), String> {
        println!("📦 [SDK] Extension '{}' installed successfully.", self.manifest().plugin_id);
        Ok(())
    }

    /// 3. Execution: Fired when the extension is activated. 
    /// Notice we pass in the `RednerEcosystemClient` (from 12.1) so they can interact safely!
    fn on_enable(&mut self, client: &RednerEcosystemClient) -> Result<(), String>;

    /// 4. Cleanup: Fired when the creator disables the plugin.
    fn on_disable(&mut self) -> Result<(), String> {
        Ok(())
    }
}