use sqlx::QueryBuilder;

use crate::{
    config::db::DbPool,
    infrastructure::sql::{
        order::{Order, OrderBy},
        pagination::Pagination,
    },
    modules::user::{
        domain::spec::{UserFilter, UserJoin, UserOrder},
        persistence::{entity::UserEntity, query::UserQuery},
    },
};

pub struct UserRepository {
    db: DbPool,
}

impl UserRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn find_all(
        &self,
        joins: &[UserJoin],
        filters: &[UserFilter],
        sort_by: UserOrder,
        order: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<UserEntity>, anyhow::Error> {
        let mut qb = QueryBuilder::new("");

        UserQuery::select(&mut qb, joins);
        UserQuery::filter(&mut qb, filters);

        // ===== ORDER =====
        let order = Order::from_str(order, sort_by);
        let orders = OrderBy(vec![order]);
        UserQuery::order(&mut qb, &orders);

        // ===== PAGINATION =====
        let pagination = Pagination::new(limit, offset);
        pagination.apply(&mut qb);

        let rows = qb
            .build_query_as::<UserEntity>()
            .fetch_all(&self.db)
            .await?;

        Ok(rows)
    }

    pub async fn count_all(
        &self,
        joins: &[UserJoin],
        filters: &[UserFilter],
    ) -> Result<i64, anyhow::Error> {
        let mut qb = QueryBuilder::new("");

        UserQuery::count(&mut qb, joins);
        UserQuery::filter(&mut qb, filters);

        let count: i64 = qb.build_query_scalar().fetch_one(&self.db).await?;

        Ok(count)
    }
}
