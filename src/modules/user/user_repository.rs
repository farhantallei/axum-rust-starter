use sqlx::QueryBuilder;

use crate::{
    config::db::DbPool,
    modules::user::{
        user_model::UserModel,
        user_query::UserQuery,
        user_spec::{UserFilter, UserJoin, UserOrder},
    },
    utils::{order::OrderBy, pagination::Pagination},
};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(
        db: &DbPool,
        joins: &[UserJoin],
        filters: &[UserFilter],
        orders: &OrderBy<UserOrder>,
        pagination: &Pagination,
    ) -> Result<Vec<UserModel>, sqlx::Error> {
        let mut qb = QueryBuilder::new("");

        UserQuery::select(&mut qb, joins);
        UserQuery::filter(&mut qb, filters);
        UserQuery::order(&mut qb, orders);

        pagination.apply(&mut qb);

        let rows = qb.build_query_as::<UserModel>().fetch_all(db).await?;

        Ok(rows)
    }

    pub async fn count_all(
        db: &DbPool,
        joins: &[UserJoin],
        filters: &[UserFilter],
    ) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new("");

        UserQuery::count(&mut qb, joins);
        UserQuery::filter(&mut qb, filters);

        let count: i64 = qb.build_query_scalar().fetch_one(db).await?;

        Ok(count)
    }
}
