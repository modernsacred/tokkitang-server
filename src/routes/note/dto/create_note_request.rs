use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub project_id: String,
    pub content: String,
    pub x: String,
    pub y: String,
}
