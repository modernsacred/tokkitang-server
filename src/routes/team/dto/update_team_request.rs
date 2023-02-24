use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
    pub description: String,
    pub thumbnail_url: Option<String>,
}
