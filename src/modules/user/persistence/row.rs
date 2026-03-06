use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};

use crate::modules::{user::domain::model::UserModel, user_role::persistence::row::UserRoleRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRow {
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

    pub role: Option<Json<UserRoleRow>>,
}

impl From<UserRow> for UserModel {
    fn from(row: UserRow) -> Self {
        UserModel {
            id: row.id,
            username: row.username,
            name: row.name,
            email: row.email,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            role: row.role.map(|k| k.0.into()),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct UserInsertRow {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub status: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<UserInsertRow> for UserModel {
    fn from(row: UserInsertRow) -> Self {
        UserModel {
            id: row.id,
            username: row.username,
            name: row.name,
            email: row.email,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            role: None,
        }
    }
}
