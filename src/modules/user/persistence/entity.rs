use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};

use crate::modules::user_role::persistence::entity::UserRoleEntity;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub email: String,
    pub status: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,

    pub role: Option<Json<UserRoleEntity>>,
}
