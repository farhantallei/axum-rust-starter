use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse<T: Serialize = serde_json::Value> {
    pub data: Vec<T>,
}

impl<T> IntoResponse for ListResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
