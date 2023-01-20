use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubLoginResponse {
    pub success: bool,
    pub access_token: String,
    pub need_signup: bool,
}
