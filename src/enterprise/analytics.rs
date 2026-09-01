use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. PRODUCTION METRICS DTOs
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectAnalytics {
    pub project_id: Uuid,
    pub total_editing_time_mins: u32,
    pub ai_assisted_percentage: f32, // e.g., 68.5% of edits done by agents
    pub revision_cycles: u32,
    pub approval_duration_hours: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrgDashboardSummary {
    pub org_id: Uuid,
    pub active_projects_count: u32,
    pub total_videos_produced: u32,
    pub average_production_hours: f32,
    pub top_performing_agent: String,
    pub bottleneck_stage: String, // e.g., "Review & Approval"
}

// ==========================================
// 2. THE CONTROL CENTER ENGINE
// ==========================================
pub struct ProductionAnalyticsEngine;

impl ProductionAnalyticsEngine {
    /// Compiles production metrics for a specific creative project.
    pub fn calculate_project_metrics(
        project_id: Uuid,
        editing_mins: u32,
        ai_ops_count: u32,
        total_ops_count: u32,
        revisions: u32,
    ) -> ProjectAnalytics {
        println!("📊 [ANALYTICS] Computing production metrics for Project {}...", project_id);

        let ai_percentage = if total_ops_count > 0 {
            (ai_ops_count as f32 / total_ops_count as f32) * 100.0
        } else {
            0.0
        };

        ProjectAnalytics {
            project_id,
            total_editing_time_mins: editing_mins,
            ai_assisted_percentage: ai_percentage,
            revision_cycles: revisions,
            approval_duration_hours: (revisions as f32) * 4.5, // Heuristic calculation
        }
    }

    /// Generates the high-level Control Center overview for the Creative Director.
    pub fn generate_org_dashboard(org_id: Uuid) -> OrgDashboardSummary {
        println!("📈 [ANALYTICS] Aggregating Enterprise Control Center dashboard for Org {}...", org_id);

        OrgDashboardSummary {
            org_id,
            active_projects_count: 14,
            total_videos_produced: 89,
            average_production_hours: 3.2,
            top_performing_agent: "TikTok Hooks Pro Agent".to_string(),
            bottleneck_stage: "Client Review & Approval".to_string(),
        }
    }
}