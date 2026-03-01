use std::time::Instant;

use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;

pub mod application;
pub mod config;
pub mod data;
pub mod infrastructure;
pub mod modules;
pub mod presentation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = data::env::init_env();

    config::logger::init_logger();
    let db = config::db::init_db(&env).await?;

    let state = presentation::state::AppState {
        started_at: Instant::now(),
        db,
    };

    let app = presentation::router::create_router()
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", env.port);
    info!("🚀 Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
