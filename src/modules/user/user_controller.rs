use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use tracing::instrument;

use crate::{
    modules::user::{
        presentation::{
            dto::{CreateUserRequest, GetUserQuery, GetUserResponse, UpdateUserRequest},
            mapper::build_user_filters,
        },
        user_repository::UserRepository,
        user_service::UserService,
    },
    presentation::{
        error::HttpError,
        http::{
            common_query::ListQueryImpl,
            common_response::{ListResponse, SingleResponse},
        },
        state::AppState,
    },
};

pub struct UserController;

impl UserController {
    #[instrument(skip(state))]
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

        Ok(ListResponse::ok(response, total))
    }

    #[instrument(skip(state))]
    pub async fn update_user_handler(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<SingleResponse<GetUserResponse>, HttpError> {
        let repo = UserRepository::new(state.db.clone());
        let service = UserService::new(repo);

        let updated = service.update_user(id, payload.user).await?;

        let response = GetUserResponse { user: updated };

        Ok(SingleResponse::ok(response))
    }

    #[instrument(skip(state))]
    pub async fn delete_user_handler(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<StatusCode, HttpError> {
        let repo = UserRepository::new(state.db.clone());
        let service = UserService::new(repo);

        service.delete_user(id).await?;

        Ok(StatusCode::NO_CONTENT)
    }

    #[instrument(skip(state))]
    pub async fn create_user_handler(
        State(state): State<AppState>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<SingleResponse<GetUserResponse>, HttpError> {
        let repo = UserRepository::new(state.db.clone());
        let service = UserService::new(repo);

        let saved = service.create_user(payload.user).await?;

        let response = GetUserResponse { user: saved };

        Ok(SingleResponse::created(response))
    }
}
