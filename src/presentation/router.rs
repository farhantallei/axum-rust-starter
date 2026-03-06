use axum::Router;

use crate::{
    modules::{health::health_route::HealthRoute, user::user_route::UserRoute},
    presentation::state::AppState,
};

pub fn create_router() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/health", HealthRoute::routes())
            .nest("/users", UserRoute::routes()),
    )
}
