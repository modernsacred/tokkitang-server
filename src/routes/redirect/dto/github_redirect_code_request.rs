use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRedirectCodeRequest {
    pub code: String,
    pub redirect_url: Option<String>,
}
