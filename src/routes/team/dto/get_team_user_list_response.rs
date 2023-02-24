use serde::{Deserialize, Serialize};

use crate::models::TeamUserAuthority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamUserListItem {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub thumbnail_url: Option<String>,
    pub authority: TeamUserAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamUserListResponse {
    pub list: Vec<GetTeamUserListItem>,
}
