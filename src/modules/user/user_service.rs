use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    modules::user::{
        query::{user_filter::UserFilter, user_join::UserJoin, user_order::UserOrder},
        user_model::UserModel,
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

    pub async fn find_all_user(
        db: &Pool<Postgres>,
        joins: &[UserJoin],
        keyword: Option<String>,
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
        let mut filters = vec![Filter::Condition(UserFilter::IsActive(true))];

        if let Some(keyword) = keyword {
            filters.push(Filter::Condition(UserFilter::NameLike(keyword)));
        }

        Filter::And(filters).apply(&mut qb);

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

        // println!("SQL: {}", qb.sql());

        let rows = qb.build_query_as::<UserModel>().fetch_all(db).await?;
        Ok(rows)
    }

    pub async fn count_all_user(
        db: &Pool<Postgres>,
        joins: &[UserJoin],
        keyword: Option<String>,
    ) -> Result<i64, sqlx::Error> {
        let mut qb = QueryBuilder::new(Self::BASE_COUNT_QUERY);

        // JOIN
        for join in joins {
            join.apply(&mut qb);
        }

        // FILTER
        let mut filters = vec![Filter::Condition(UserFilter::IsActive(true))];

        if let Some(keyword) = keyword {
            filters.push(Filter::Condition(UserFilter::NameLike(keyword)));
        }

        Filter::And(filters).apply(&mut qb);

        // println!("SQL: {}", qb.sql());

        let count: i64 = qb.build_query_scalar().fetch_one(db).await?;
        Ok(count)
    }
}
