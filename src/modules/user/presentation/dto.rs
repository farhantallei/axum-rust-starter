use serde::{Deserialize, Serialize};

use crate::{
    modules::user::domain::model::{UpdateUserPayload, UserModel, UserPayload},
    presentation::http::common_query::ListQuery,
};

// ===== GET =====
#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    #[serde(flatten)]
    pub base: ListQuery,
    pub actived: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    #[serde(flatten)]
    pub user: UserModel,
}

// ===== CREATE =====
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    #[serde(flatten)]
    pub user: UserPayload,
}

// ===== UPDATE =====
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(flatten)]
    pub user: UpdateUserPayload,
}
