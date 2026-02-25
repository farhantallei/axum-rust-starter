use tracing::instrument;

use crate::{
    config::db::DbPool,
    modules::user::{
        user_model::UserModel,
        user_repository::UserRepository,
        user_spec::{UserFilter, UserJoin, UserOrder},
    },
    utils::{
        order::{Order, OrderBy},
        pagination::Pagination,
    },
};

#[derive(Clone)]
pub struct UserService;

impl UserService {
    #[instrument(skip(joins, filters))]
    pub async fn find_all_user_with_count(
        db: &DbPool,
        joins: &[UserJoin],
        filters: &[UserFilter],
        sort_by: Option<String>,
        order: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<(Vec<UserModel>, i64), anyhow::Error> {
        // JOIN
        let mut all_joins = vec![UserJoin::UserRole];

        all_joins.extend(joins.iter().cloned());

        // FILTER
        let mut all_filters = vec![UserFilter::IsDeleted(false)];

        all_filters.extend(filters.iter().cloned());

        // ORDER
        let order_field = match sort_by.as_deref() {
            Some("name") => UserOrder::Name,
            _ => UserOrder::Id,
        };

        let order = match order.as_deref() {
            Some("desc") => Order::Desc(order_field),
            _ => Order::Asc(order_field),
        };

        // PAGINATION
        let pagination = Pagination { limit, offset };

        let total = UserRepository::count_all(db, &all_joins, &all_filters).await?;

        let data = UserRepository::find_all(
            db,
            &all_joins,
            &all_filters,
            &OrderBy(vec![order]),
            &pagination,
        )
        .await?;

        Ok((data, total))
    }
}
