use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProjectListItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProjectListResponse {
    pub list: Vec<GetProjectListItem>,
}
