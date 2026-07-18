use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::runtime::graph::graph::RuntimeGraph;
use crate::runtime::graph::node::NodeStatus;
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::registry::RunnerRegistry;
use crate::runtime::jobs::store::JobStore;
use crate::runtime::jobs::models::JobStatus;

#[derive(Clone)]
pub struct GraphDispatcher {
    registry: RunnerRegistry,
    store: Option<JobStore>, // Optional so we can run tests without a DB if needed
}

impl GraphDispatcher {
    pub fn new(registry: RunnerRegistry, store: Option<JobStore>) -> Self {
        Self { registry, store }
    }

    /// Executes the entire graph and persists state to NeonDB
    pub async fn execute_graph(&self, job_id: Option<Uuid>, graph: Arc<Mutex<RuntimeGraph>>, context: ExecutionContext) -> Result<(), String> {
        println!("🚀 Dispatcher: Starting execution for graph...");

        // Mark job as Running in DB
        self.sync_db(job_id, JobStatus::Running, &graph).await;

        loop {
            let runnable_node_ids = {
                let g = graph.lock().await;
                let ready = g.get_runnable_nodes();
                
                if ready.is_empty() {
                    let all_done = g.nodes.values().all(|n| {
                        matches!(n.status, NodeStatus::Completed | NodeStatus::Cached | NodeStatus::Failed(_))
                    });
                    
                    if all_done {
                        println!("✅ Dispatcher: Graph execution finished completely!");
                        self.sync_db(job_id, JobStatus::Completed, &graph).await;
                        break;
                    }
                }
                ready
            };

            if runnable_node_ids.is_empty() {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            for node_id in runnable_node_ids {
                {
                    let mut g = graph.lock().await;
                    g.update_node_status(&node_id, NodeStatus::Running);
                }
                
                // Sync DB when a node starts running
                self.sync_db(job_id, JobStatus::Running, &graph).await;

                let node = {
                    let g = graph.lock().await;
                    g.nodes.get(&node_id).cloned().unwrap()
                };

                println!("⚡ Dispatcher: Executing Node -> [{}] ({:?})", node.id, node.node_type);

                if let Some(runner) = self.registry.get(&node.node_type) {
                    match runner.execute(&node, &context).await {
                        Ok(outputs) => {
                            let mut g = graph.lock().await;
                            if let Some(n) = g.nodes.get_mut(&node_id) {
                                n.status = NodeStatus::Completed;
                                n.outputs = outputs;
                            }
                            println!("🟢 Node [{}] Completed Successfully!", node_id);
                        }
                        Err(e) => {
                            let mut g = graph.lock().await;
                            g.update_node_status(&node_id, NodeStatus::Failed(e.clone()));
                            println!("🔴 Node [{}] Failed: {}", node_id, e);
                            self.sync_db(job_id, JobStatus::Failed(e.clone()), &graph).await;
                            return Err(format!("Graph aborted due to failure in {}: {}", node_id, e));
                        }
                    }
                    
                    // Sync DB when a node finishes
                    self.sync_db(job_id, JobStatus::Running, &graph).await;

                } else {
                    let err_msg = format!("No runner registered for {:?}", node.node_type);
                    let mut g = graph.lock().await;
                    g.update_node_status(&node_id, NodeStatus::Failed(err_msg.clone()));
                    self.sync_db(job_id, JobStatus::Failed(err_msg.clone()), &graph).await;
                    return Err(err_msg);
                }
            }
        }

        Ok(())
    }

    /// Helper to cleanly sync state to DB if a store is provided
    async fn sync_db(&self, job_id: Option<Uuid>, status: JobStatus, graph_arc: &Arc<Mutex<RuntimeGraph>>) {
        if let (Some(store), Some(id)) = (&self.store, job_id) {
            let graph = graph_arc.lock().await;
            if let Err(e) = store.update_job_state(id, &status, &graph).await {
                println!("⚠️ DB Sync Error for Job {}: {}", id, e);
            }
        }
    }
}