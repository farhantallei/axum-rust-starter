use sqlx::{Postgres, QueryBuilder};

use crate::{
    modules::user::user_spec::{UserFilter, UserJoin, UserOrder},
    utils::{filter::ApplyFilter, order::ApplyOrder},
};

impl ApplyFilter for UserFilter {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
        match self {
            UserFilter::NameLike(val) => {
                qb.push(" u.name ILIKE ");
                qb.push_bind(format!("%{}%", val));
            }
            UserFilter::Email(val) => {
                qb.push(" u.email = ");
                qb.push_bind(val.clone());
            }
            UserFilter::IsActive(active) => {
                qb.push(" u.status = ");
                qb.push_bind(*active);
            }
            UserFilter::IsDeleted(deleted) => {
                if *deleted {
                    qb.push(" u.deleted_at IS NOT NULL");
                } else {
                    qb.push(" u.deleted_at IS NULL");
                }
            }
        }
    }
}

impl UserJoin {
    pub fn apply(&self, _qb: &mut QueryBuilder<Postgres>) {}
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
