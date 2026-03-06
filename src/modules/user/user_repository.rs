use sqlx::QueryBuilder;

use crate::{
    config::db::DbPool,
    infrastructure::sql::{
        order::{Order, OrderBy},
        pagination::Pagination,
    },
    modules::user::{
        domain::{
            model::{UpdateUserPayload, UserModel, UserPayload},
            spec::{UserFilter, UserJoin, UserOrder},
        },
        persistence::{
            mutation::UserMutation,
            query::UserQuery,
            row::{UserInsertRow, UserRow},
        },
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
    ) -> Result<Vec<UserModel>, anyhow::Error> {
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

        let rows = qb.build_query_as::<UserRow>().fetch_all(&self.db).await?;

        Ok(rows.into_iter().map(Into::into).collect())
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

    pub async fn insert(&self, payload: UserPayload) -> Result<UserModel, anyhow::Error> {
        let mut qb = QueryBuilder::new("");
        UserMutation::insert(&mut qb, &payload);
        let row = qb
            .build_query_as::<UserInsertRow>()
            .fetch_one(&self.db)
            .await?;
        Ok(row.into())
    }

    pub async fn update(
        &self,
        id: i32,
        payload: UpdateUserPayload,
    ) -> Result<Option<UserModel>, anyhow::Error> {
        let mut qb = QueryBuilder::new("");
        UserMutation::update(&mut qb, id, &payload);
        let row = qb
            .build_query_as::<UserInsertRow>()
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(Into::into))
    }

    pub async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        let mut qb = QueryBuilder::new("");
        UserMutation::delete(&mut qb, id);
        Ok(())
    }
}
