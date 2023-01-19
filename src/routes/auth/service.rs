use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::Extension;
use epoch_timestamp::Epoch;
use std::error::Error;

use crate::{models::User, utils::jwt};

pub struct AuthService {
    _client: Extension<Arc<Client>>,
}

impl AuthService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { _client: client }
    }

    pub fn get_access_token(&self, user_id: String) -> String {
        let epoch = (Epoch::now() + Epoch::day(1)) as usize;

        jwt::sign(epoch, user_id)
    }

    pub fn get_github_access_token(code: String) -> Result<String, Box<dyn Error>> {
        let client_secret = std::env::var("GITHUB_SECRET").unwrap();
        let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();
        let redirect_url = "https://tokkitang.com/redirect/github-login".to_owned();

        struct GetAccessTokenRequestBody {
            client_secret: String,
            client_id: String,
            redirect_url: String,
            code: String,
        };

        let body = redirect_url {
            client_secret,
            client_id,
            redirect_url,
            code,
        };

        let client = reqwest::Client::new();
        let result = client
            .post("https://github.com/login/oauth/access_token")
            .body(body)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct GetAccessTokenResponseBody {
            access_token: String,
        };

        let result = result.text().await?;

        let result: GetAccessTokenResponseBody = serde_json::from_str(result.as_str())?;

        Ok(result.access_token)
    }

    pub fn get_github_user(access_token: String) -> Result<String, Box<dyn Error>> {
        unimplemented!();
    }
}
