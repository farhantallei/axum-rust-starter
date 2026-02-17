use sqlx::{Postgres, QueryBuilder};

pub enum UserJoin {}

impl UserJoin {
    pub fn apply(&self, _qb: &mut QueryBuilder<Postgres>) {}
}
