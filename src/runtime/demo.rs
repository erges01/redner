use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::path::PathBuf;
use std::env;

use crate::runtime::graph::node::{NodeType, RuntimeNode};
use crate::runtime::graph::graph::RuntimeGraph;
use crate::runtime::registry::RunnerRegistry;
use crate::runtime::context::execution_context::ExecutionContext;
use crate::runtime::executor::dispatcher::GraphDispatcher;
use crate::runtime::jobs::mock::MockRunner;
use crate::runtime::jobs::store::JobStore;

// 👇 NEW IMPORTS for the Real Voice Engine
use crate::runtime::jobs::voice::VoiceRunner;
use crate::runtime::providers::elevenlabs::ElevenLabsProvider;

pub async fn run_graph_demo() {
    println!("\n===========================================");
    println!("🎬 REDNER RUNTIME GRAPH: ELEVENLABS EDITION");
    println!("===========================================\n");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to NeonDB");
    
    let store = JobStore::new(pool);

    // 2. Setup Registry
    let mut registry = RunnerRegistry::new();
    
    // Keep Narration as Mock for now
    registry.register(MockRunner { node_type: NodeType::Narration });
    
    // 👇 THE SWAP: Use the Real ElevenLabs Provider for Voice!
    let elevenlabs_provider = Arc::new(ElevenLabsProvider::new());
    registry.register(VoiceRunner::new(elevenlabs_provider));

    // Keep LipSync as Mock for now
    registry.register(MockRunner { node_type: NodeType::LipSync });

    // 3. Setup Context
    let project_id = Uuid::new_v4();
    let context = ExecutionContext::new(project_id, PathBuf::from("./temp"));

    // 4. Build the Graph
    let mut graph = RuntimeGraph::new("demo_voice_blueprint");
    graph.add_node(RuntimeNode::new("step_1_narration", NodeType::Narration, vec![]));
    graph.add_node(RuntimeNode::new("step_3_lipsync", NodeType::LipSync, vec!["step_2_voice".to_string()]));
    
    // Let's pass a custom text input into the voice node!
    let mut voice_node = RuntimeNode::new("step_2_voice", NodeType::Voice, vec!["step_1_narration".to_string()]);
    voice_node.inputs.insert("text".to_string(), serde_json::json!("Welcome to Redner. The ultimate AI rendering engine built in Rust. Let's make a masterpiece."));
    graph.add_node(voice_node);

    // 5. Create Job
    let job_id = store.create_job(project_id, &graph).await.expect("Failed to create job in DB");
    println!("💾 Created Job ID in NeonDB: {}", job_id);

    // 6. Dispatch!
    let dispatcher = GraphDispatcher::new(registry, Some(store));
    let graph_arc = Arc::new(Mutex::new(graph));

    let _ = dispatcher.execute_graph(Some(job_id), graph_arc, context).await;
    
    println!("\n===========================================");
    println!("🏁 DEMO COMPLETE - CHECK YOUR /temp FOLDER!");
    println!("===========================================\n");
}