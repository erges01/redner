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

// Scene Runtime Imports (6.5)
use crate::runtime::jobs::scene::SceneRunner;
use crate::runtime::providers::mock_scene::MockSceneProvider;

// 🎞️ Composition Runtime Imports (6.6)
use crate::runtime::jobs::composition::CompositionRunner;
use crate::runtime::providers::local_composition::LocalCompositionProvider;

// 🎥 NEW: Render Imports (6.7 & 6.8)
use crate::runtime::jobs::render::RenderRunner;
use crate::runtime::providers::local_render::LocalRenderProvider;

pub async fn run_graph_demo() {
    println!("\n===========================================");
    println!("🎬 REDNER RUNTIME GRAPH: END-TO-END DEMO");
    println!("===========================================\n");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to NeonDB");
    
    let store = JobStore::new(pool);

    // 2. Setup Registry
    let mut registry = RunnerRegistry::new();

    // Narration
    registry.register(MockRunner { node_type: NodeType::Narration });
    
    // Voice & LipSync (Mocked for token savings & smooth execution)
    registry.register(MockRunner { node_type: NodeType::Voice });
    registry.register(MockRunner { node_type: NodeType::LipSync });

    // Scene Generation (6.5)
    let scene_provider = Arc::new(MockSceneProvider::new());
    registry.register(SceneRunner::new(scene_provider));

    // 🎞️ Composition Engine (6.6)
    let comp_provider = Arc::new(LocalCompositionProvider::new());
    registry.register(CompositionRunner::new(comp_provider));

    // 🎥 NEW: Render Engine (6.7 & 6.8)
    let render_provider = Arc::new(LocalRenderProvider::new());
    registry.register(RenderRunner::new(render_provider));

    // 3. Setup Context
    let project_id = Uuid::new_v4();
    let context = ExecutionContext::new(project_id, PathBuf::from("./temp"));

    // 4. Build the Graph
    let mut graph = RuntimeGraph::new("demo_full_pipeline");
    
    // Node 1: Narration
    graph.add_node(RuntimeNode::new("step_1_narration", NodeType::Narration, vec![]));
    
    // Node 2: Voice
    graph.add_node(RuntimeNode::new("step_2_voice", NodeType::Voice, vec!["step_1_narration".to_string()]));

    // Node 3: LipSync
    graph.add_node(RuntimeNode::new("step_3_lipsync", NodeType::LipSync, vec!["step_2_voice".to_string()]));

    // Node 4: Scene Node (Independent start)
    let mut scene_node = RuntimeNode::new("step_4_scene", NodeType::Scene, vec![]);
    scene_node.inputs.insert("prompt".to_string(), serde_json::json!("Sleek Rust terminal code editor background with neon lighting"));
    graph.add_node(scene_node);

    // 🎞️ Node 5: The Master Composition Node
    let mut comp_node = RuntimeNode::new(
        "step_5_composition", 
        NodeType::Composition, 
        vec!["step_2_voice".to_string(), "step_3_lipsync".to_string(), "step_4_scene".to_string()]
    );
    comp_node.inputs.insert("scene_path".to_string(), serde_json::json!("./temp/step_4_scene_scene.json"));
    comp_node.inputs.insert("voice_path".to_string(), serde_json::json!("./temp/mock_voice.mp3"));
    comp_node.inputs.insert("lipsync_path".to_string(), serde_json::json!("./temp/mock_visemes.json"));
    graph.add_node(comp_node);

    // 🎥 Node 6: The Final Render Node
    // This node WILL NOT start until Composition is completely finished!
    let mut render_node = RuntimeNode::new(
        "step_6_render", 
        NodeType::Render, 
        vec!["step_5_composition".to_string()]
    );
    render_node.inputs.insert("timeline_path".to_string(), serde_json::json!("./temp/step_5_composition_timeline.json"));
    graph.add_node(render_node);

    // 5. Create Job
    let job_id = store.create_job(project_id, &graph).await.expect("Failed to create job in DB");
    println!("Created Job ID in NeonDB: {}", job_id);

    // 6. Dispatch!
    let dispatcher = GraphDispatcher::new(registry, Some(store));
    let graph_arc = Arc::new(Mutex::new(graph));

    let _ = dispatcher.execute_graph(Some(job_id), graph_arc, context).await;
    
    println!("\n===========================================");
    println!("🎉 PHASE 6 COMPLETE - MP4 GENERATED IN /temp!");
    println!("===========================================\n");
}