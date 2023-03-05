use serde::{Deserialize, Serialize};

use crate::models::Column;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityRequest {
    pub logical_name: String,
    pub physical_name: String,
    pub comment: String,
    pub columns: Vec<Column>,
    pub x: String,
    pub y: String,
}
