use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupGithubRequest {
    pub nickname: String,
    pub email: String,
    pub thumbnail_url: Option<String>,
    pub access_token: String,
}
