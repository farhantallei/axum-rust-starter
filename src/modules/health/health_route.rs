use axum::{routing::get, Router};

use crate::{modules::health::health_controller::HealthController, presentation::state::AppState};

pub struct HealthRoute;

impl HealthRoute {
    pub fn routes() -> Router<AppState> {
        Router::new().route("/", get(HealthController::healthcheck_handler))
    }
}
