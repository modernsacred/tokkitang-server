use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNoteItem {
    pub id: String,
    pub content: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNoteResponse {
    pub data: GetNoteItem,
}
