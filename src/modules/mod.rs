use axum::Router;

use crate::{
    modules::{health::health_route, user::user_route},
    shared::state::AppState,
};

pub mod health;
pub mod user;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/health", health_route::router())
                .nest("/users", user_route::router()),
        )
        .with_state(state)
}
