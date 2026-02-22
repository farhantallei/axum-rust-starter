use serde::Serialize;

use crate::modules::user_role::user_role_model::UserRoleModel;

#[derive(Debug, Serialize)]
pub struct GetUserRoleResponse {
    #[serde(flatten)]
    pub user_role: UserRoleModel,
}
