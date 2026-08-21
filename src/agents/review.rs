use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. THE QA FEEDBACK MODEL
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QaStatus {
    Approved,
    Rejected {
        reason: String,
        suggested_fix: String,
        target_agent: String, // e.g., "Editor_V1"
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewCritique {
    pub critique_id: Uuid,
    pub task_id: Uuid, // The task that was reviewed
    pub status: QaStatus,
}

// ==========================================
// 2. THE REVIEW ENGINE
// ==========================================
pub struct ReviewEngine;

impl ReviewEngine {
    /// Simulates the Review Agent analyzing a completed task
    pub fn evaluate_task(task_description: &str) -> ReviewCritique {
        println!("🔍 [REVIEW ENGINE] Analyzing output for task: '{}'", task_description);
        
        // Mocking a QA check: If the task was editing, maybe we find a pacing issue!
        let status = if task_description.contains("visuals and audio") {
            println!("❌ [REVIEW ENGINE] Flaw detected: 'Audio peaks above target at 00:04'.");
            QaStatus::Rejected {
                reason: "Audio peaking at 00:04".to_string(),
                suggested_fix: "Normalize audio track 2 to -6dB".to_string(),
                target_agent: "Editor_V1".to_string(),
            }
        } else {
            println!("✅ [REVIEW ENGINE] Output meets quality standards.");
            QaStatus::Approved
        };

        ReviewCritique {
            critique_id: Uuid::new_v4(),
            task_id: Uuid::new_v4(),
            status,
        }
    }
}