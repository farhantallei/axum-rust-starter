use axum::Router;

use crate::{
    modules::{health::health_route, user::user_route},
    presentation::state::AppState,
};

pub fn create_router() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/health", health_route::router())
            .nest("/users", user_route::router()),
    )
}
