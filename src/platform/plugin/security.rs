use serde::{Deserialize, Serialize};

// ==========================================
// 1. PLUGIN PERMISSIONS
// What is this third-party plugin allowed to do?
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Permission {
    ReadTimeline,
    WriteTimeline,
    ReadAssets,
    ExportVideo,
    NetworkAccess,
}

// ==========================================
// 2. THE PLUGIN MANIFEST
// The identity card of a third-party extension.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub plugin_id: String,       // e.g., "com.adesope.tiktok-captions"
    pub version: String,         // e.g., "1.0.0"
    pub publisher: String,       // e.g., "Adesope"
    pub requested_permissions: Vec<Permission>,
}

// ==========================================
// 3. THE SECURITY GATEKEEPER
// ==========================================
pub struct PluginSecurityManager;

impl PluginSecurityManager {
    /// Validates if a plugin has permission to perform an action.
    /// E.g., The "Color Grader" plugin shouldn't have NetworkAccess.
    pub fn authorize_action(manifest: &PluginManifest, required_permission: Permission) -> Result<(), String> {
        if manifest.requested_permissions.contains(&required_permission) {
            Ok(())
        } else {
            let err_msg = format!(
                "🛑 [SECURITY ERROR] Plugin '{}' attempted an unauthorized action: {:?}", 
                manifest.plugin_id, required_permission
            );
            println!("{}", err_msg);
            Err(err_msg)
        }
    }
}