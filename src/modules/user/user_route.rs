use axum::{routing::get, Router};

use crate::{modules::user::user_controller::find_all_user_handler, presentation::state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(find_all_user_handler))
}
