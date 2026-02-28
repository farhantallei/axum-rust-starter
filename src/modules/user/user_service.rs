use tracing::instrument;

use crate::{
    application::error::ApplicationError,
    modules::user::{
        domain::spec::{UserFilter, UserJoin, UserOrder},
        persistence::entity::UserEntity,
        presentation::error::UserError,
        user_repository::UserRepository,
    },
};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    #[instrument(skip(self, joins, filters))]
    pub async fn find_all_user_with_count(
        &self,
        joins: &[UserJoin],
        filters: &[UserFilter],
        sort_by: Option<&str>,
        order: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<(Vec<UserEntity>, i64), ApplicationError> {
        // ===== JOIN =====
        let mut effective_joins = Vec::with_capacity(1 + joins.len());
        effective_joins.push(UserJoin::UserRole);
        effective_joins.extend_from_slice(joins);

        // ===== FILTER =====
        let mut effective_filters = Vec::with_capacity(1 + filters.len());
        effective_filters.push(UserFilter::IsDeleted(false));
        effective_filters.extend_from_slice(filters);

        // ===== ORDER =====
        let order_field = match sort_by {
            Some("name") => UserOrder::Name,
            _ => UserOrder::Id,
        };

        // ===== EXECUTE PARALLEL QUERY =====
        let (total_res, data_res) = tokio::join!(
            self.repo.count_all(&effective_joins, &effective_filters),
            self.repo.find_all(
                &effective_joins,
                &effective_filters,
                order_field,
                order,
                limit,
                offset,
            )
        );

        let total = total_res.map_err(UserError::Unexpected)?;
        let data = data_res.map_err(UserError::Unexpected)?;

        Ok((data, total))
    }
}
