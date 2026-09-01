use serde::{Deserialize, Serialize};

// ==========================================
// 1. ENTERPRISE ROLES
// The identities of both humans and machines.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum EnterpriseRole {
    Owner,
    Admin,
    CreativeDirector,
    Editor,
    Reviewer,
    Viewer,
    AiAgent, // 🤖 Machine identity!
}

// ==========================================
// 2. RESOURCE CAPABILITIES
// The explicit actions that can be performed in the system.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Capability {
    ManageOrganization,
    ManageBilling,
    InviteMembers,
    DeleteProject,
    ReadTimeline,
    ModifyTimeline,
    GenerateCaptions,
    ReviewAndApprove,
}

// ==========================================
// 3. THE PERMISSION ENGINE
// Evaluates if a role is authorized to perform a capability.
// ==========================================
pub struct PermissionEngine;

impl PermissionEngine {
    /// Strict authorization gateway for all Enterprise actions.
    pub fn authorize(role: &EnterpriseRole, action: Capability) -> Result<(), String> {
        let is_authorized = match role {
            EnterpriseRole::Owner => true, // God mode
            EnterpriseRole::Admin => matches!(
                action, 
                Capability::ManageOrganization | Capability::ManageBilling | Capability::InviteMembers | Capability::ReadTimeline
            ),
            EnterpriseRole::CreativeDirector => !matches!(
                action, 
                Capability::ManageBilling // Can do everything except billing
            ),
            EnterpriseRole::Editor => matches!(
                action, 
                Capability::ReadTimeline | Capability::ModifyTimeline | Capability::GenerateCaptions
            ),
            EnterpriseRole::Reviewer => matches!(
                action, 
                Capability::ReadTimeline | Capability::ReviewAndApprove
            ),
            EnterpriseRole::Viewer => matches!(
                action, 
                Capability::ReadTimeline
            ),
            // 🤖 THE MACHINE SCOPE: Strict boundaries for autonomous agents
            EnterpriseRole::AiAgent => matches!(
                action, 
                Capability::ReadTimeline | Capability::ModifyTimeline | Capability::GenerateCaptions
            ),
        };

        if is_authorized {
            println!("✅ [RBAC] Access Granted: {:?} -> {:?}", role, action);
            Ok(())
        } else {
            let err = format!("🛑 [RBAC] SECURITY BLOCK: {:?} is NOT allowed to {:?}", role, action);
            println!("{}", err);
            Err(err)
        }
    }
}