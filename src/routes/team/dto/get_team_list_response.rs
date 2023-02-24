use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamListItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamListResponse {
    pub list: Vec<GetTeamListItem>,
}
