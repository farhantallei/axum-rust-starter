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
        // ===== JOIN =====
        let mut effective_joins = Vec::with_capacity(1 + joins.len());
        effective_joins.push(UserJoin::UserRole);
        effective_joins.extend_from_slice(joins);

        // ===== FILTER =====
        let mut effective_filters = Vec::with_capacity(1 + filters.len());
        effective_filters.push(UserFilter::IsDeleted(false));
        effective_filters.extend_from_slice(filters);

        // ===== ORDER =====
        let order_field = match sort_by.as_deref() {
            Some("name") => UserOrder::Name,
            _ => UserOrder::Id,
        };
        let order = Order::from_str(order.as_deref(), order_field);
        let effective_orders = OrderBy(vec![order]);

        // ===== PAGINATION =====
        let pagination = Pagination::new(limit, offset);

        // ===== EXECUTE PARALLEL QUERY =====
        let (total_res, data_res) = tokio::join!(
            UserRepository::count_all(db, &effective_joins, &effective_filters),
            UserRepository::find_all(
                db,
                &effective_joins,
                &effective_filters,
                &effective_orders,
                &pagination,
            )
        );

        let total = total_res?;
        let data = data_res?;

        Ok((data, total))
    }
}
