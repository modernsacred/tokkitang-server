use serde::{Deserialize, Serialize};

use crate::models::TeamUserAuthority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub thumbnail_url: Option<String>,
    pub authority: TeamUserAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamResponse {
    pub data: GetTeamItem,
}
