use uuid::Uuid;
use crate::agents::tasks::{TaskGraph, TaskNode, TaskStatus};
use crate::agents::models::AgentRole;

pub struct CreativeDirector;

impl CreativeDirector {
    /// The Director receives a human goal and decomposes it into a graph of specialized tasks.
    pub fn decompose_goal(goal: &str) -> TaskGraph {
        println!("🎬 [DIRECTOR] Analyzing Goal: {}", goal);
        println!("🎬 [DIRECTOR] Decomposing into a Task Graph...");

        // Task 1: Write the Script
        let script_task_id = Uuid::new_v4();
        let script_task = TaskNode {
            task_id: script_task_id,
            description: "Write a high-retention script matching the creator's pacing".to_string(),
            required_role: AgentRole::ScriptSpecialist,
            dependencies: vec![], // Zero dependencies. Can start immediately!
            status: TaskStatus::Pending,
        };

        // Task 2: Edit the Video
        let editor_task_id = Uuid::new_v4();
        let editor_task = TaskNode {
            task_id: editor_task_id,
            description: "Cut visuals and audio to match the script timing".to_string(),
            required_role: AgentRole::Editor,
            dependencies: vec![script_task_id], // 🛑 MUST wait for the Script Specialist!
            status: TaskStatus::Pending,
        };

        // Task 3: QA & Review
        let review_task_id = Uuid::new_v4();
        let review_task = TaskNode {
            task_id: review_task_id,
            description: "QA the final timeline for pacing errors and dead air".to_string(),
            required_role: AgentRole::Reviewer,
            dependencies: vec![editor_task_id], // 🛑 MUST wait for the Editor!
            status: TaskStatus::Pending,
        };

        TaskGraph {
            graph_id: Uuid::new_v4(),
            goal: goal.to_string(),
            nodes: vec![script_task, editor_task, review_task],
        }
    }
}