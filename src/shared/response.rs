use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
}
