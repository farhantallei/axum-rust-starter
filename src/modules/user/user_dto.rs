use serde::{Deserialize, Serialize};

use crate::{
    modules::user::{user_model::UserModel, user_spec::UserFilter},
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

impl GetUserQuery {
    pub fn to_filters(&self) -> Vec<UserFilter> {
        let mut filters = vec![];

        if let Some(keyword) = &self.keyword() {
            filters.push(UserFilter::NameLike(keyword.clone()));
        }

        if let Some(actived) = self.actived {
            filters.push(UserFilter::IsActive(actived));
        }

        filters
    }
}

#[derive(Debug, Serialize)]
pub struct GetUserResponse {
    #[serde(flatten)]
    pub user: UserModel,
}
