use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::data::env::Env;

const MAX_DB_CONNECTIONS: u32 = 5;

pub type DbPool = Pool<Postgres>;

pub async fn init_db(env: &Env) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(&env.database_url)
        .await?;

    Ok(pool)
}
