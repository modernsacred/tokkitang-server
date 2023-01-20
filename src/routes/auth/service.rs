use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::Extension;
use epoch_timestamp::Epoch;
use reqwest::header;
use std::error::Error;

use crate::{
    models::User,
    utils::{http, jwt},
};

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

    pub async fn get_github_access_token(&self, code: String) -> Option<String> {
        let headers = http::default_header();

        let client_secret = std::env::var("GITHUB_SECRET").unwrap();
        let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();

        #[derive(serde::Serialize)]
        struct GetAccessTokenRequestBody {
            client_secret: String,
            client_id: String,
            code: String,
        }

        let body = GetAccessTokenRequestBody {
            client_secret,
            client_id,
            code,
        };

        let body = serde_json::to_string(&body).unwrap();

        let client = reqwest::Client::new();
        let result = client
            .post("https://github.com/login/oauth/access_token")
            .body(body)
            .headers(headers)
            .send()
            .await
            .ok()?;

        #[derive(serde::Deserialize)]
        struct GetAccessTokenResponseBody {
            access_token: String,
            #[allow(dead_code)]
            token_type: String,
            #[allow(dead_code)]
            scope: String,
        }

        let result = result.text().await.ok()?;

        let result = serde_json::from_str::<GetAccessTokenResponseBody>(result.as_str()).ok()?;

        Some(result.access_token)
    }

    pub async fn get_github_user(&self, access_token: String) -> Option<String> {
        let mut headers = http::default_header();
        let bearer = format!("Bearer {}", access_token);
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(bearer.as_str()).unwrap(),
        );
        headers.insert("User-Agent", header::HeaderValue::from_static("tokkitang"));

        let client = reqwest::Client::new();
        let result = client
            .get("https://api.github.com/user")
            .headers(headers)
            .send()
            .await
            .ok()?;

        let result = result.text().await.ok()?;
        println!("result: {}", result);

        Some("".into())
    }
}
