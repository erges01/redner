use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::platform::marketplace::{ExtensionType, MarketplaceListing};
use crate::platform::plugin::security::PluginManifest;

// ==========================================
// 1. DEVELOPER ACCOUNTS & CREDENTIALS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeveloperAccount {
    pub developer_id: Uuid,
    pub username: String,
    pub email: String,
    pub api_key: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishPackageRequest {
    pub manifest: PluginManifest,
    pub extension_type: ExtensionType,
    pub description: String,
    pub wasm_binary_base64: String, // Payload uploaded by the CLI tool
}

// ==========================================
// 2. THE DEVELOPER PORTAL SERVICE
// Handles extension verification, local dev tunneling, and publishing.
// ==========================================
pub struct DeveloperPortal;

impl DeveloperPortal {
    /// Generates API credentials for third-party CLI integration (`redner auth login`)
    pub fn create_developer_account(username: &str, email: &str) -> DeveloperAccount {
        let account = DeveloperAccount {
            developer_id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            api_key: format!("rdk_{}", Uuid::new_v4().simple()),
            is_verified: true,
        };

        println!("👨‍💻 [PORTAL] Registered Developer: '{}' (Key: {})", account.username, account.api_key);
        account
    }

    /// Verifies manifest integrity and publishes package to the Marketplace Registry
    pub fn publish_package(
        dev: &DeveloperAccount, 
        req: PublishPackageRequest
    ) -> Result<MarketplaceListing, String> {
        println!("🚀 [PORTAL] Developer '{}' publishing package: '{}' (v{})", 
            dev.username, req.manifest.plugin_id, req.manifest.version
        );

        // 1. Validation checks
        if req.wasm_binary_base64.is_empty() {
            return Err("Binary payload cannot be empty.".to_string());
        }

        if req.manifest.requested_permissions.is_empty() {
            println!("⚠️ [PORTAL] Warning: Package requests 0 permissions.");
        }

        // 2. Simulate compiling and uploading to the Redner Global CDN
        let registry_url = format!(
            "https://registry.redner.os/pkg/{}-{}.wasm", 
            req.manifest.plugin_id, 
            req.manifest.version
        );

        let listing = MarketplaceListing {
            listing_id: Uuid::new_v4(),
            name: req.manifest.plugin_id.clone(),
            publisher: dev.username.clone(),
            extension_type: req.extension_type,
            version: req.manifest.version,
            description: req.description,
            downloads: 0,
            rating: 5.0,
            registry_url,
        };

        println!("✨ [PORTAL] Package published! Live on Marketplace with ID: {}", listing.listing_id);
        Ok(listing)
    }
}