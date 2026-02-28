use crate::{
    modules::user::{domain::spec::UserFilter, presentation::dto::GetUserQuery},
    presentation::http::common_query::ListQueryImpl,
};

impl ListQueryImpl for GetUserQuery {
    fn start(&self) -> Option<i32> {
        self.base.start
    }
    fn limit(&self) -> Option<i32> {
        self.base.limit
    }
    fn keyword(&self) -> Option<&str> {
        self.base.keyword.as_deref()
    }

    fn sort_by(&self) -> Option<&str> {
        self.base.sort_by.as_deref()
    }

    fn order(&self) -> Option<&str> {
        self.base.order.as_deref()
    }
}

pub fn build_user_filters(query: &GetUserQuery) -> Vec<UserFilter> {
    let mut filters = Vec::with_capacity(2);

    if let Some(keyword) = &query.base.keyword {
        filters.push(UserFilter::NameLike(keyword.to_string()));
    }

    if let Some(actived) = query.actived {
        filters.push(UserFilter::IsActive(actived));
    }

    filters
}
