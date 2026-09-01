use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::intelligence::memory::{CreativeDecision, DecisionAction};
use std::collections::HashMap;

// ==========================================
// 1. THE PATTERN STATUS
// Enforces the "Creator Agency" rule.
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PatternStatus {
    PendingApproval, // Prompts the UI: "I noticed this. Save it?"
    Accepted,        // Becomes a hard rule in the CreatorProfile
    Rejected,        // The creator said "No, that was just a coincidence."
}

// ==========================================
// 2. THE DETECTED PATTERN
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedPattern {
    pub pattern_id: Uuid,
    pub description: String,
    pub confidence: f32,
    pub occurrences: u32,
    pub status: PatternStatus,
}

// ==========================================
// 3. THE PATTERN DETECTOR
// Scans historical ledgers for recurring creator behavior.
// ==========================================
pub struct PatternDetector;

impl PatternDetector {
    /// Scans the history of explicit creative decisions to find recurring habits.
    pub fn scan_decision_history(decisions: &[CreativeDecision]) -> Vec<DetectedPattern> {
        println!("🔍 [PATTERN DETECTOR] Scanning decision history for recurring habits...");
        
        let mut rejection_counts: HashMap<String, u32> = HashMap::new();
        let mut new_patterns = Vec::new();

        // 1. Tally up repeated actions across all past projects
        for decision in decisions {
            if let DecisionAction::Rejected = decision.action {
                *rejection_counts.entry(decision.target_component.clone()).or_insert(0) += 1;
            }
        }

        // 2. Identify statistically significant habits (e.g., >= 3 occurrences)
        for (component, count) in rejection_counts {
            if count >= 3 {
                println!("💡 [PATTERN DETECTOR] Insight: Creator rejected '{}' {} times.", component, count);
                
                // Bayesian-lite confidence math: 3 times is ~70%, 10 times is 99%
                let confidence = (0.5 + ((count as f32).ln() * 0.15)).clamp(0.5, 0.99);

                new_patterns.push(DetectedPattern {
                    pattern_id: Uuid::new_v4(),
                    description: format!("You frequently remove '{}'. Would you like me to stop suggesting it?", component),
                    confidence,
                    occurrences: count,
                    status: PatternStatus::PendingApproval, // 🛑 Waiting for human consent!
                });
            }
        }

        if new_patterns.is_empty() {
            println!("💤 [PATTERN DETECTOR] No new patterns reached the significance threshold.");
        }

        new_patterns
    }
}