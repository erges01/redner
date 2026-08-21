mod api;
mod app;
mod db;
mod dto;
mod error;
mod models;
mod repos;
mod services;
mod ai;
pub mod performance;
pub mod runtime;
pub mod platform;
pub mod cloud;
pub mod agents; 
pub mod live;

use std::{env, net::SocketAddr, sync::Arc};

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// --- 🛠️ NEW: CORS Imports ---
use axum::http::{Method, header::{ACCEPT, CONTENT_TYPE}};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    app::{router::create_router, state::AppState},
    cloud::api::build_cloud_router, // 👈 2. Import the Cloud Master Router
    db::pool::create_pool,
    repos::{
        asset_repo::AssetRepo,
        project_repo::ProjectRepo,
        timeline_repo::TimelineRepo,
    },
    services::{
        project_service::ProjectService,
        timeline_service::TimelineService,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env variables
    dotenv().ok();

    // Setup logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Connect to Postgres
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = create_pool(&database_url).await?;

    // Initialize Repositories
    let project_repo = ProjectRepo::new(pool.clone());
    let asset_repo = AssetRepo::new(pool.clone());
    let timeline_repo = TimelineRepo::new(pool.clone());

    // Initialize Services
    let timeline_service = TimelineService::new(project_repo.clone(), timeline_repo);
    let project_service = ProjectService::new(
        project_repo,
        asset_repo,
        timeline_service.clone(),
    );

    // Build App State
    let state = AppState {
        db: pool,
        project_service: Arc::new(project_service),
        timeline_service: Arc::new(timeline_service),
    };

    // --- 🛠️ NEW: Configure CORS ---
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, ACCEPT]);

    // Create Router, MERGE the cloud router, and attach the CORS layer
    let app = create_router(state)
        .merge(build_cloud_router()) // 👈 3. Mount the Phase 8 APIs!
        .layer(cors);

   // Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🚀 Redner Backend listening on http://{}", addr);

    // Run your demos in the background (DO THIS BEFORE AXUM::SERVE)
    tokio::spawn(crate::runtime::demo::run_graph_demo());
    tokio::spawn(crate::platform::plugin::demo::run_wasm_demo());

    // Start the server (this blocks forever, so it MUST be the very last thing)
    axum::serve(listener, app).await?;

    Ok(())
}