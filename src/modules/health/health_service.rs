use sysinfo::System;
use tracing::instrument;

use crate::{
    application::error::ApplicationError,
    modules::health::{
        health_repository::HealthRepository,
        presentation::{dto::Cpu, error::HealthError},
        utils::format_bytes,
    },
};

pub struct HealthService {
    repo: HealthRepository,
}

impl HealthService {
    pub fn new(repo: HealthRepository) -> Self {
        Self { repo }
    }

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

    #[instrument(skip(self))]
    pub async fn get_db_connection(&self) -> Result<bool, ApplicationError> {
        let result = self
            .repo
            .get_db_connection()
            .await
            .map_err(HealthError::Unexpected)?;

        Ok(result)
    }
}
