use axum::{routing::get, Router};

use crate::{
    modules::health::health_controller::healthcheck_handler, presentation::state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(healthcheck_handler))
}
