use std::time::Instant;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub started_at: Instant,
    pub db: Pool<Postgres>,
}
