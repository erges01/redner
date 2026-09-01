use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// 1. REVIEW STATES & TIMELINE COMMENTS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReviewStatus {
    PendingReview,
    ChangesRequested,
    Approved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineComment {
    pub comment_id: Uuid,
    pub project_id: Uuid,
    pub author_id: Uuid,
    pub timestamp_ms: u64,          // The exact millisecond on the timeline
    pub target_clip_id: Option<Uuid>, // Optional binding to a specific clip
    pub body: String,
    pub is_resolved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewSubmission {
    pub submission_id: Uuid,
    pub project_id: Uuid,
    pub version_number: u64,
    pub status: ReviewStatus,
    pub comments: Vec<TimelineComment>,
}

// ==========================================
// 2. THE REVIEW & APPROVAL SERVICE
// ==========================================
pub struct ReviewEngine;

impl ReviewEngine {
    /// Submits a project version for client or creative director review.
    pub fn submit_for_review(project_id: Uuid, version_number: u64) -> ReviewSubmission {
        println!("📤 [REVIEW] Project {} (v{}) submitted for stakeholder review.", project_id, version_number);
        
        ReviewSubmission {
            submission_id: Uuid::new_v4(),
            project_id,
            version_number,
            status: ReviewStatus::PendingReview,
            comments: Vec::new(),
        }
    }

    /// Drops a timeline-aware comment directly onto a specific timestamp.
    pub fn add_timestamp_comment(
        submission: &mut ReviewSubmission,
        author_id: Uuid,
        timestamp_ms: u64,
        clip_id: Option<Uuid>,
        body: &str,
    ) -> TimelineComment {
        println!("💬 [REVIEW] New feedback at {}ms: \"{}\"", timestamp_ms, body);

        let comment = TimelineComment {
            comment_id: Uuid::new_v4(),
            project_id: submission.project_id,
            author_id,
            timestamp_ms,
            target_clip_id: clip_id,
            body: body.to_string(),
            is_resolved: false,
        };

        submission.comments.push(comment.clone());
        submission.status = ReviewStatus::ChangesRequested;
        comment
    }

    /// Approves the review submission, unlocking the export pipeline.
    pub fn approve_submission(submission: &mut ReviewSubmission) -> Result<(), String> {
        let unresolved_count = submission.comments.iter().filter(|c| !c.is_resolved).count();
        
        if unresolved_count > 0 {
            let err = format!("🛑 [REVIEW] Cannot approve: {} unresolved comment(s) remaining.", unresolved_count);
            println!("{}", err);
            return Err(err);
        }

        submission.status = ReviewStatus::Approved;
        println!("🎉 [REVIEW] Submission APPROVED! Export lock released.");
        Ok(())
    }
}