use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

// ===== LIST RESPONSE =====
#[derive(Serialize)]
pub struct ListResponse<T: Serialize = serde_json::Value> {
    pub data: Vec<T>,
    #[serde(rename = "recordsFiltered")]
    pub records_filtered: i64,
    #[serde(skip)]
    pub status: StatusCode,
}

impl<T: Serialize> ListResponse<T> {
    pub fn ok(data: Vec<T>, records_filtered: i64) -> Self {
        Self {
            data,
            records_filtered,
            status: StatusCode::OK,
        }
    }

    pub fn created(data: Vec<T>, records_filtered: i64) -> Self {
        Self {
            data,
            records_filtered,
            status: StatusCode::CREATED,
        }
    }

    pub fn no_content(data: Vec<T>, records_filtered: i64) -> Self {
        Self {
            data,
            records_filtered,
            status: StatusCode::NO_CONTENT,
        }
    }
}

impl<T> IntoResponse for ListResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let body = Json(self);

        (status, body).into_response()
    }
}

// ===== SINGLE RESPONSE =====
#[derive(Serialize)]
pub struct SingleResponse<T: Serialize = serde_json::Value> {
    pub data: T,
    #[serde(skip)]
    pub status: StatusCode,
}

impl<T: Serialize> SingleResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            data,
            status: StatusCode::OK,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            data,
            status: StatusCode::CREATED,
        }
    }

    pub fn no_content(data: T) -> Self {
        Self {
            data,
            status: StatusCode::NO_CONTENT,
        }
    }
}

impl<T> IntoResponse for SingleResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let body = Json(self);

        (status, body).into_response()
    }
}
