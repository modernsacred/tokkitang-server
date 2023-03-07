use serde::{Deserialize, Serialize};

use crate::models::TeamUserAuthority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeAuthorityRequest {
    pub user_id: String,
    pub authority: TeamUserAuthority,
}
