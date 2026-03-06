use serde::Serialize;

use crate::modules::user_role::persistence::row::UserRoleRow;

#[derive(Debug, Serialize)]
pub struct GetUserRoleResponse {
    #[serde(flatten)]
    pub user_role: UserRoleRow,
}
