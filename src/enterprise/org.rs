use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE ORGANIZATION (The New Root Node)
// e.g., "Redner Studio", "Acme Agency"
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub org_id: Uuid,
    pub name: String,
    pub owner_user_id: Uuid,
}

// ==========================================
// 2. TEAMS (Sub-divisions)
// e.g., "Script Writers", "Motion Designers"
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub team_id: Uuid,
    pub org_id: Uuid,
    pub name: String,
}

// ==========================================
// 3. ORGANIZATION MEMBERS
// Maps a transient User to an Organization
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgMember {
    pub membership_id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub team_ids: Vec<Uuid>, // A user can belong to multiple teams
}

// ==========================================
// 4. THE ORGANIZATION ENGINE
// Manages the lifecycle of the enterprise boundary
// ==========================================
pub struct OrgEngine;

impl OrgEngine {
    /// Provisions a new enterprise organization
    pub fn create_organization(name: &str, owner_id: Uuid) -> Organization {
        println!("🏢 [ENTERPRISE] Provisioning new Organization: '{}'", name);
        Organization {
            org_id: Uuid::new_v4(),
            name: name.to_string(),
            owner_user_id: owner_id,
        }
    }

    /// Subdivides an organization into specialized creative teams
    pub fn create_team(org_id: Uuid, team_name: &str) -> Team {
        println!("👥 [ENTERPRISE] Creating Team '{}' under Org {}", team_name, org_id);
        Team {
            team_id: Uuid::new_v4(),
            org_id,
            name: team_name.to_string(),
        }
    }

    /// Adds a user to the organization
    pub fn add_member(org_id: Uuid, user_id: Uuid) -> OrgMember {
        println!("🤝 [ENTERPRISE] Adding User {} to Organization {}", user_id, org_id);
        OrgMember {
            membership_id: Uuid::new_v4(),
            org_id,
            user_id,
            team_ids: Vec::new(),
        }
    }
}