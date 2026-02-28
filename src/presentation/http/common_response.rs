use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse<T: Serialize = serde_json::Value> {
    pub data: Vec<T>,
    #[serde(rename = "recordsFiltered")]
    pub records_filtered: i64,
}

impl<T> IntoResponse for ListResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
