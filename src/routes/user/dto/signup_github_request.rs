use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupGithubRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,

    pub access_token: String,
}
