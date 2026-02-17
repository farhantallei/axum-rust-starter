use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub start: Option<i64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub limit: Option<i64>,

    pub keyword: Option<String>,

    pub sort_by: Option<String>,

    pub order: Option<String>,
}

pub trait ListQueryImpl {
    fn start(&self) -> Option<i64>;
    fn limit(&self) -> Option<i64>;
    fn keyword(&self) -> Option<String>;
    fn sort_by(&self) -> Option<String>;
    fn order(&self) -> Option<String>;
}
