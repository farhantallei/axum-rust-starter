use std::time::Instant;

use crate::config::db::DbPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub started_at: Instant,
    pub db: DbPool,
}
