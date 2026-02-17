use axum::extract::{Query, State};

use crate::{
    modules::user::{
        user_dto::{GetUserQuery, GetUserResponse},
        user_service::UserService,
    },
    shared::{error::AppError, request::ListQueryImpl, response::ListResponse, state::AppState},
};

pub async fn find_all_user_handler(
    State(state): State<AppState>,
    Query(params): Query<GetUserQuery>,
) -> Result<ListResponse<GetUserResponse>, AppError> {
    let data = UserService::find_all_user(
        &state.db,
        &[],
        params.keyword().clone(),
        params.sort_by.clone(),
        params.order(),
        params.limit(),
        params.start(),
    )
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    let total = UserService::count_all_user(&state.db, &[], params.keyword())
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let response: Vec<GetUserResponse> = data
        .into_iter()
        .map(|item| GetUserResponse { user: item })
        .collect();

    Ok(ListResponse {
        data: response,
        records_filtered: total,
    })
}
