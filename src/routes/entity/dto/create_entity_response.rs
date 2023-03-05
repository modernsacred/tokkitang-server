use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityResponse {
    pub success: bool,
    pub entity_id: String,
}
