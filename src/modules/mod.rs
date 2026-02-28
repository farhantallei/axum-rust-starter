use axum::Router;

use crate::{
    modules::{health::health_route, user::user_route},
    presentation::state::AppState,
};

pub mod health;
pub mod user;
pub mod user_role;

pub fn create_router() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/health", health_route::router())
            .nest("/users", user_route::router()),
    )
}
