use std::time::Instant;

use crate::{config::db::init_db, state::AppState};

pub mod config;
pub mod data;
pub mod modules;
pub mod shared;
pub mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = data::env::init_env();

    let db = init_db(&env).await?;

    let state = AppState {
        started_at: Instant::now(),
        db,
    };

    let app = modules::create_router(state);

    let addr = format!("0.0.0.0:{}", env.port);
    println!("🚀 Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
