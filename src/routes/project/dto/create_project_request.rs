use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub team_id: String,
    pub name: String,
    pub description: String,
    pub thumbnail_url: Option<String>,
}
