use serde::Serialize;

use crate::modules::user_role::persistence::entity::UserRoleEntity;

#[derive(Debug, Serialize)]
pub struct GetUserRoleResponse {
    #[serde(flatten)]
    pub user_role: UserRoleEntity,
}
