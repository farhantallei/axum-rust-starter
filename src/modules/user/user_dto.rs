use serde::{Deserialize, Serialize};

use crate::{
    modules::user::user_model::UserModel,
    shared::request::{ListQuery, ListQueryImpl},
};

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    #[serde(flatten)]
    pub base: ListQuery,
    pub sort_by: Option<String>,
}

impl ListQueryImpl for GetUserQuery {
    fn start(&self) -> Option<i64> {
        self.base.start
    }
    fn limit(&self) -> Option<i64> {
        self.base.limit
    }
    fn keyword(&self) -> Option<String> {
        self.base.keyword.clone()
    }

    fn order(&self) -> Option<String> {
        self.base.order.clone()
    }
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    #[serde(flatten)]
    pub user: UserModel,
}
