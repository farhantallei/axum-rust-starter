use sqlx::{Postgres, QueryBuilder};

#[derive(Clone, Copy, Debug, Default)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Pagination {
    pub fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
        if let Some(limit) = self.limit {
            qb.push(" LIMIT ");
            qb.push_bind(limit);

            let offset = self.offset.unwrap_or(0);
            qb.push(" OFFSET ");
            qb.push_bind(offset);
        }
    }
}
