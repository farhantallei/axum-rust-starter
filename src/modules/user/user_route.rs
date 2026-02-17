use axum::{Router, routing::get};

use crate::{modules::user::user_controller::find_all_user_handler, shared::state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(find_all_user_handler))
}
