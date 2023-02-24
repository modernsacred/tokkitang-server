use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyInfoResponse {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub thumbnail_url: Option<String>,
}
