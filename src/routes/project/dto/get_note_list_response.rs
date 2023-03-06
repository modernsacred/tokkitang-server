use serde::{Deserialize, Serialize};

use crate::models::Column;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNoteListItem {
    pub id: String,
    pub content: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNoteListResponse {
    pub list: Vec<GetNoteListItem>,
}
