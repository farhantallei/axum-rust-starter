use std::{
    env::consts::{ARCH, OS},
    process,
};

use axum::{Json, extract::State};
use chrono::Utc;
use sysinfo::System;

use crate::{
    modules::health::{
        health_dto::GetHealthResponse,
        health_service::{get_cpus, get_db_connection, get_memory},
    },
    shared::error::AppError,
    shared::state::AppState,
};

pub async fn healthcheck_handler(
    State(state): State<AppState>,
) -> Result<Json<GetHealthResponse>, AppError> {
    let mut sys = System::new_all();

    sys.refresh_cpu_all();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    sys.refresh_cpu_all();

    sys.refresh_memory();

    let db_status = get_db_connection(&state.db).await;

    let response = GetHealthResponse {
        status: "ok".to_string(),
        db: match db_status {
            true => "connected",
            false => "disconnected",
        }
        .to_string(),
        timestamp: Utc::now().to_rfc3339(),
        uptime: state.started_at.elapsed().as_secs(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: OS.to_string(),
        arch: ARCH.to_string(),
        pid: process::id().to_string(),
        cpus: get_cpus(&sys),
        memory: get_memory(&sys),
    };

    Ok(Json(response))
}
