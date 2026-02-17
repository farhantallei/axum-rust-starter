use serde::{Deserialize, Serialize};

use crate::{
    modules::user::user_model::UserModel,
    shared::request::{ListQuery, ListQueryImpl},
};

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    #[serde(flatten)]
    pub base: ListQuery,
    pub actived: Option<bool>,
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

    fn sort_by(&self) -> Option<String> {
        self.base.sort_by.clone()
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
