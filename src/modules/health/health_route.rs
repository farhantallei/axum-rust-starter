use axum::{Router, routing::get};

use crate::{modules::health::health_controller::healthcheck_handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(healthcheck_handler))
}
