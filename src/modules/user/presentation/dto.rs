use serde::{Deserialize, Serialize};

use crate::{
    modules::user::persistence::entity::UserEntity, presentation::http::common_query::ListQuery,
};

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    #[serde(flatten)]
    pub base: ListQuery,
    pub actived: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    #[serde(flatten)]
    pub user: UserEntity,
}
