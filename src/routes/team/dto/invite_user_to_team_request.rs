use serde::{Deserialize, Serialize};

use crate::models::TeamUserAuthority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteUserToTeamRequest {
    pub user_id: String,
    pub authority: TeamUserAuthority,
}
