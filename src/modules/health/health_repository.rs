use sysinfo::System;

use crate::config::db::DbPool;

pub struct HealthRepository {
    db: DbPool,
}

impl HealthRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn get_sys() -> System {
        let mut sys = System::new_all();

        sys.refresh_cpu_all();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        sys.refresh_cpu_all();

        sys.refresh_memory();

        sys
    }

    pub async fn get_db_connection(&self) -> Result<bool, anyhow::Error> {
        let result = sqlx::query("SELECT 1").fetch_one(&self.db).await;

        Ok(result.is_ok())
    }
}
