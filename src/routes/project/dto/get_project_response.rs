use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProjectItem {
    pub id: String,
    pub description: String,
    pub name: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProjectResponse {
    pub data: GetProjectItem,
}
