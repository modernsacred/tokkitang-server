use serde::{Deserialize, Serialize};

use crate::models::Column;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntityItem {
    pub id: String,
    pub logical_name: String,
    pub physical_name: String,
    pub comment: String,
    pub columns: Vec<Column>,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntityResponse {
    pub data: GetEntityItem,
}
