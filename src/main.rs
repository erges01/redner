mod api;
mod app;
mod db;
mod dto;
mod error;
mod models;
mod repos;
mod services;

use std::{env, net::SocketAddr, sync::Arc};

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app::{router::create_router, state::AppState},
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

    // Create Router
    let app = create_router(state);

    // Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🚀 Redner Backend listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}