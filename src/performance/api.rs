use axum::{
    routing::get, // 👈 Removed 'post' to fix the unused import warning
    Json, Router,
};
use serde_json::{json, Value};
use crate::{
    app::state::AppState, // 👈 NEW: Import your AppState
    performance::profile::{
        PerformanceProfile, EnergyLevel, ExpressionPreset, GestureStyle, CameraFraming, SpeakingStyle
    }
};

/// GET /api/performance/profiles
/// Fetches all performance profiles for the user
async fn get_profiles() -> Json<Vec<PerformanceProfile>> {
    // 🚧 TODO: Query your database here
    let default_profile = PerformanceProfile {
        id: "default-1".to_string(),
        name: "Default Creator".to_string(),
        energy: EnergyLevel::Medium,
        default_expression: ExpressionPreset::Neutral,
        eye_contact_level: 80,
        gesture_style: GestureStyle::Teacher,
        preferred_camera: CameraFraming::Medium,
        speaking_style: SpeakingStyle {
            pace: 1.0,
            confidence: "high".to_string(),
            pause_length: "medium".to_string(),
        },
    };

    Json(vec![default_profile])
}

/// POST /api/performance/profiles
/// Creates or updates a performance profile
async fn save_profile(Json(payload): Json<PerformanceProfile>) -> Json<Value> {
    // 🚧 TODO: Insert or Update the database here
    println!("🎬 Saving Performance Profile: {} (Energy: {:?})", payload.name, payload.energy);

    Json(json!({
        "status": "success",
        "message": "Performance profile saved successfully",
        "profile_id": payload.id
    }))
}

/// The router to mount in your main app
// 👈 NEW: Added <AppState> so it matches the parent router
pub fn performance_routes() -> Router<AppState> {
    Router::new()
        .route("/profiles", get(get_profiles).post(save_profile))
}