use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::data::env::Env;

const MAX_DB_CONNECTIONS: u32 = 5;

pub async fn init_db(env: &Env) -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(MAX_DB_CONNECTIONS)
        .connect(&env.database_url)
        .await?;

    Ok(pool)
}
