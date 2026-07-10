use axum::{Json, response::IntoResponse, http::StatusCode};
use super::models::{CopilotRequest, AIResponsePayload, AIOperation};

pub async fn copilot_chat(
    // 🛠️ FIX: Accept raw JSON first so we can debug it!
    Json(raw_payload): Json<serde_json::Value>, 
) -> impl IntoResponse {
    
    // Attempt to deserialize manually to catch the exact error
    let payload: CopilotRequest = match serde_json::from_value(raw_payload.clone()) {
        Ok(p) => p,
        Err(e) => {
            println!("🚨 AXUM 422 ERROR PREVENTED!");
            println!("🚨 Reason: {}", e);
            println!("🚨 Raw JSON received: {:#?}", raw_payload);
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    };

    println!("🤖 AI REQUEST RECEIVED!");
    println!("User Prompt: {}", payload.prompt);
    println!("Current Playhead: {}ms", payload.context.playback.current_time_ms);
    println!("Total Tracks: {}", payload.context.project.track_count);
    
    if let Some(clip_id) = &payload.context.selection.selected_clip_id {
        println!("User is currently selecting clip: {}", clip_id);
    } else {
        println!("No clip currently selected.");
    }

    // FAKE THE LLM (Plumbing Check)
    let simulated_response = AIResponsePayload {
        thoughts: "I am a Rust backend. I have received your timeline state. To prove I can control your editor, I am seeking your playhead to 5 seconds (5000ms).".to_string(),
        operations: vec![
            AIOperation::Seek { time_ms: 5000.0 },
        ],
    };

    println!("🤖 SENDING COMMANDS TO FRONTEND...\n");
    Json(simulated_response).into_response()
}