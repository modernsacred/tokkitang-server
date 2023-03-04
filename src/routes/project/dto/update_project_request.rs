use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: String,
    pub description: String,
    pub thumbnail_url: Option<String>,
    pub x: String,
    pub y: String,
}
