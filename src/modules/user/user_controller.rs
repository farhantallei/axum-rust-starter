use axum::extract::{Query, State};

use crate::{
    modules::user::{
        user_dto::{GetUserQuery, GetUserResponse},
        user_service::UserService,
    },
    shared::{error::AppError, request::ListQueryImpl, response::ListResponse, state::AppState},
};

#[tracing::instrument(skip(state))]
pub async fn find_all_user_handler(
    State(state): State<AppState>,
    Query(params): Query<GetUserQuery>,
) -> Result<ListResponse<GetUserResponse>, AppError> {
    let data = UserService::find_all_user_with_count(
        &state.db,
        &[],
        &params.to_filters(),
        params.sort_by(),
        params.order(),
        params.limit(),
        params.start(),
    )
    .await?;

    Ok(data)
}
