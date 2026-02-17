use sqlx::{Pool, Postgres};
use sysinfo::System;
use tracing::instrument;

use crate::modules::health::{health_dto::Cpu, health_util::format_bytes};

#[derive(Clone)]
pub struct HealthService;

impl HealthService {
    #[instrument]
    pub fn get_cpus(sys: &System) -> Vec<Cpu> {
        sys.cpus()
            .iter()
            .map(|c| Cpu {
                model: c.brand().to_string(),
                usage: c.cpu_usage().round(),
            })
            .collect()
    }

    #[instrument]
    pub fn get_memory(sys: &System) -> String {
        format!(
            "{} / {}",
            format_bytes(sys.used_memory()),
            format_bytes(sys.total_memory())
        )
    }

    #[instrument]
    pub async fn get_db_connection(db: &Pool<Postgres>) -> bool {
        let result = sqlx::query("SELECT 1").fetch_one(db).await;

        result.is_ok()
    }
}
