use sqlx::{Pool, Postgres, QueryBuilder};
use tracing::instrument;

use crate::{
    modules::user::{
        user_model::UserModel,
        user_spec::{UserFilter, UserJoin, UserOrder},
    },
    utils::{
        filter::Filter,
        order::{Order, OrderBy},
    },
};

#[derive(Clone)]
pub struct UserService;

impl UserService {
    pub const BASE_QUERY: &str = "SELECT u.* FROM users u";
    pub const BASE_COUNT_QUERY: &str = "SELECT COUNT(u.*) FROM users u";

    #[instrument(skip(joins, filters))]
    pub async fn find_all_user(
        db: &Pool<Postgres>,
        joins: &[UserJoin],
        filters: &[UserFilter],
        sort_by: Option<String>,
        order: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserModel>, sqlx::Error> {
        let mut qb = QueryBuilder::new(Self::BASE_QUERY);

        // JOIN
        for j in joins {
            j.apply(&mut qb);
        }

        // FILTER
        let mut all_filters = vec![Filter::Condition(UserFilter::IsDeleted(false))];

        all_filters.extend(filters.iter().cloned().map(Filter::Condition));

        Filter::And(all_filters).apply(&mut qb);

        // ORDER
        let order_field = match sort_by.as_deref() {
            Some("name") => UserOrder::Name,
            _ => UserOrder::Id,
        };

        let order = match order.as_deref() {
            Some("desc") => Order::Desc(order_field),
            _ => Order::Asc(order_field),
        };

        OrderBy(vec![order]).apply(&mut qb);

        if let Some(limit) = limit {
            qb.push(" LIMIT ");
            qb.push_bind(limit as i32);

            let offset = offset.unwrap_or(0);
            qb.push(" OFFSET ");
            qb.push_bind(offset as i32);
        }

        let rows = qb.build_query_as::<UserModel>().fetch_all(db).await?;
        Ok(rows)
    }

    #[instrument(skip(joins, filters))]
    pub async fn count_all_user(
        db: &Pool<Postgres>,
        joins: &[UserJoin],
        filters: &[UserFilter],
    ) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new(Self::BASE_COUNT_QUERY);

        // JOIN
        for join in joins {
            join.apply(&mut qb);
        }

        // FILTER
        let mut all_filters = vec![Filter::Condition(UserFilter::IsDeleted(false))];

        all_filters.extend(filters.iter().cloned().map(Filter::Condition));

        Filter::And(all_filters).apply(&mut qb);

        let count: i64 = qb.build_query_scalar().fetch_one(db).await?;
        Ok(count)
    }
}
