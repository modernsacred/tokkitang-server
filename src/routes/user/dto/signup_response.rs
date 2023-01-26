use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupResponse {
    pub success: bool,
    pub email_duplicate: bool,
    pub access_token: String,
}
