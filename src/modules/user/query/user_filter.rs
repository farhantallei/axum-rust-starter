use crate::utils::filter::ApplyFilter;
use sqlx::QueryBuilder;

#[derive(Clone)]
pub enum UserFilter {
    NameLike(String),
    Email(String),
    IsActive(bool),
    IsDeleted(bool),
}

impl ApplyFilter for UserFilter {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, sqlx::Postgres>) {
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
