use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::user_role::domain::model::UserRoleModel;

#[derive(Debug, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub status: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub role: Option<UserRoleModel>,
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub name: Option<String>,
}
