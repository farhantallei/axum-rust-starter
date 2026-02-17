use sqlx::{Postgres, QueryBuilder};

use crate::utils::order::ApplyOrder;

#[derive(Clone)]
pub enum UserOrder {
    Id,
    Name,
    Email,
    CreatedAt,
}

impl ApplyOrder for UserOrder {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
        match self {
            Self::Id => qb.push("u.id"),
            Self::Name => qb.push("u.name"),
            Self::Email => qb.push("u.email"),
            Self::CreatedAt => qb.push("u.created_at"),
        };
    }
}
