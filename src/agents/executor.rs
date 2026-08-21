use crate::agents::tasks::{TaskGraph, TaskStatus};
use crate::agents::models::{AgentContext, AgentRole};
use crate::agents::specialists::{AgentExecutor, ScriptSpecialist, EditorAgent, ReviewAgent};
use crate::agents::review::{ReviewEngine, QaStatus}; // 👈 The QA engine imported

// ==========================================
// THE GRAPH EXECUTOR
// The engine that drives the multi-agent system.
// ==========================================
pub struct GraphExecutor;

impl GraphExecutor {
    /// Executes a TaskGraph sequentially based on dependencies.
    /// (In a production system, independent nodes run concurrently via tokio::spawn)
    pub async fn execute_graph(graph: &mut TaskGraph, context: &AgentContext) {
        println!("🚀 [EXECUTOR] Booting AI Pipeline for Goal: '{}'", graph.goal);

        for node in &mut graph.nodes {
            println!("--------------------------------------------------");
            println!("⏳ [EXECUTOR] Preparing task: {}", node.description);
            
            // Mark the task as running
            node.status = TaskStatus::InProgress;

            // Route the task to the correct specialized agent
            let execution_result = match node.required_role {
                AgentRole::ScriptSpecialist => {
                    let agent = ScriptSpecialist;
                    agent.execute(node, context).await
                },
                AgentRole::Editor => {
                    let agent = EditorAgent;
                    agent.execute(node, context).await
                },
                AgentRole::Reviewer => {
                    let agent = ReviewAgent;
                    agent.execute(node, context).await
                },
                AgentRole::CreativeDirector => {
                    println!("⚠️ [EXECUTOR] Director manages, it does not execute leaf tasks.");
                    Err("Invalid role for execution".to_string())
                },
                _ => {
                    println!("⚠️ [EXECUTOR] No specialist registered for role: {:?}", node.required_role);
                    Err("Specialist not found".to_string())
                }
            };

            // Handle the result and update the Graph state
            match execution_result {
                Ok(status) => {
                    node.status = status;
                    println!("✅ [EXECUTOR] Task Completed!");

                    // 🛑 NEW: THE REVIEW LOOP 🛑
                    // We don't QA the Reviewer itself, that creates an infinite loop!
                    if node.required_role != AgentRole::Reviewer {
                        let critique = ReviewEngine::evaluate_task(&node.description);
                        
                        match critique.status {
                            QaStatus::Approved => {
                                println!("🟢 [QA] Task Approved by Review Agent.");
                            },
                            QaStatus::Rejected { reason, suggested_fix, target_agent } => {
                                println!("🔴 [QA] REJECTED! Reason: {}", reason);
                                println!("🔄 [EXECUTOR] Routing back to {} with fix: {}", target_agent, suggested_fix);
                                
                                // In a real system, we append a new "Revision" TaskNode to the graph here!
                                node.status = TaskStatus::Pending; // Force a retry
                            }
                        }
                    }
                },
                Err(error) => {
                    node.status = TaskStatus::Failed(error.clone());
                    println!("❌ [EXECUTOR] Task Failed: {}. Halting pipeline.", error);
                    break; // Stop the graph if a critical dependency fails
                }
            }
        }

        println!("--------------------------------------------------");
        println!("🎉 [EXECUTOR] Task Graph Execution Finished!");
    }
}