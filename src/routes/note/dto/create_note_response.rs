use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteResponse {
    pub success: bool,
    pub note_id: String,
}
