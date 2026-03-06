use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::modules::user_role::domain::model::UserRoleModel;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRoleRow {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<UserRoleRow> for UserRoleModel {
    fn from(row: UserRoleRow) -> Self {
        UserRoleModel {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
