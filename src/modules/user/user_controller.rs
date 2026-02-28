use axum::extract::{Query, State};

use crate::{
    modules::user::{
        presentation::{
            dto::{GetUserQuery, GetUserResponse},
            mapper::build_user_filters,
        },
        user_repository::UserRepository,
        user_service::UserService,
    },
    presentation::{
        error::HttpError,
        http::{common_query::ListQueryImpl, common_response::ListResponse},
        state::AppState,
    },
};

#[tracing::instrument(skip(state))]
pub async fn find_all_user_handler(
    State(state): State<AppState>,
    Query(params): Query<GetUserQuery>,
) -> Result<ListResponse<GetUserResponse>, HttpError> {
    let repo = UserRepository::new(state.db.clone());
    let service = UserService::new(repo);

    let filters = build_user_filters(&params);

    let (data, total) = service
        .find_all_user_with_count(
            &[],
            &filters,
            params.sort_by(),
            params.order(),
            params.limit(),
            params.start(),
        )
        .await?;

    let response = data
        .into_iter()
        .map(|item| GetUserResponse { user: item })
        .collect();

    Ok(ListResponse {
        data: response,
        records_filtered: total,
    })
}
