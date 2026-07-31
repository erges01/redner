use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ==========================================
// 1. ORGANIZATION ROLES (RBAC)
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OrgRole {
    Owner,      // Full billing and destruction rights
    Admin,      // Can invite/kick members
    Producer,   // Can create projects and manage assets
    Editor,     // Can edit projects, cannot delete them
    Reviewer,   // Can only leave timeline comments (Phase 8.4)
    Viewer,     // Read-only access
}

// ==========================================
// 2. THE ORGANIZATION MODEL
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub org_id: Uuid,
    pub name: String,         // e.g., "MrBeast Studios", "Marques Brownlee LLC"
    pub slug: String,         // e.g., "mrbeast-studios"
    pub stripe_customer_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ==========================================
// 3. THE MEMBERSHIP JOIN TABLE
// Connects Creators to Organizations
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgMember {
    pub org_id: Uuid,
    pub creator_id: Uuid,
    pub role: OrgRole,
    pub joined_at: DateTime<Utc>,
}

// ==========================================
// 4. WORKSPACE CONTEXT EXTRACTOR
// ==========================================
/// When a user makes an API call, they might be acting as themselves OR as their agency.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActingContext {
    Personal(Uuid), // Operating in their private workspace
    Organization { 
        org_id: Uuid, 
        role: OrgRole 
    },
}

impl ActingContext {
    /// Helper to verify if the user has permission to edit a project
    pub fn can_edit_project(&self) -> bool {
        match self {
            ActingContext::Personal(_) => true,
            ActingContext::Organization { role, .. } => {
                matches!(role, OrgRole::Owner | OrgRole::Admin | OrgRole::Producer | OrgRole::Editor)
            }
        }
    }
}