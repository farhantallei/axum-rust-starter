use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub start: Option<i32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    pub limit: Option<i32>,

    pub keyword: Option<String>,

    pub sort_by: Option<String>,

    pub order: Option<String>,
}

pub trait ListQueryImpl {
    fn start(&self) -> Option<i32>;
    fn limit(&self) -> Option<i32>;
    fn keyword(&self) -> Option<&str>;
    fn sort_by(&self) -> Option<&str>;
    fn order(&self) -> Option<&str>;
}
