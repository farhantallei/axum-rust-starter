use axum::{
    routing::{get, put},
    Router,
};

use crate::{modules::user::user_controller::UserController, presentation::state::AppState};

pub struct UserRoute;

impl UserRoute {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route(
                "/",
                get(UserController::find_all_user_handler)
                    .post(UserController::create_user_handler),
            )
            .route(
                "/{id}",
                put(UserController::update_user_handler)
                    .delete(UserController::delete_user_handler),
            )
    }
}
