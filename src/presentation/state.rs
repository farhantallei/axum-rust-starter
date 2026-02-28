use std::time::Instant;

use crate::config::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub started_at: Instant,
    pub db: DbPool,
}
