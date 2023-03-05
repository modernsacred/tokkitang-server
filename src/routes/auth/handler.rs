use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    models::{InsertUser, User},
    routes::{auth::dto::GithubAccessTokenResponse, user::UserService},
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        GithubAccessTokenRequest, GithubLoginRequest, GithubLoginResponse, LoginRequest,
        LoginResponse,
    },
    AuthService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/login", post(login))
        .route("/login/github", post(login_github))
        .route("/access-token/github", post(get_github_access_token));

    app
}

async fn login(
    client: Extension<Arc<Client>>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let auth_service = AuthService::new(client.clone());
    let user_service = UserService::new(client);

    let mut response = LoginResponse {
        access_token: "".into(),
        success: false,
    };

    let email = body.email;
    let password = body.password;

    match user_service.find_by_email(email).await {
        Ok(user) => {
            if let Some(user) = user {
                let salt = user.password_salt;

                let hashed_password = hash_password(&password, &salt);

                if hashed_password == user.password {
                    response.success = true;

                    let user_id = user.id;
                    let access_token = auth_service.get_access_token(&user_id);

                    response.access_token = access_token;
                }
            } else {
                println!("유저 없음");
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    Json(response).into_response()
}

async fn login_github(
    client: Extension<Arc<Client>>,
    Json(body): Json<GithubLoginRequest>,
) -> impl IntoResponse {
    let auth_service = AuthService::new(client.clone());
    let user_service = UserService::new(client);

    let github_user = auth_service.get_github_user(&body.access_token).await;

    let github_user = match github_user {
        Some(github_user) => github_user,
        None => {
            let response = GithubLoginResponse {
                success: false,
                access_token: "".into(),
                need_signup: false,
            };

            return Json(response).into_response();
        }
    };

    match user_service
        .find_by_github_id(github_user.id.to_string())
        .await
    {
        Ok(user) => {
            if let Some(user) = user {
                let user_id = user.id;
                let access_token = auth_service.get_access_token(&user_id);

                let response = GithubLoginResponse {
                    success: true,
                    access_token,
                    need_signup: false,
                };

                Json(response).into_response()
            } else {
                let response = GithubLoginResponse {
                    success: false,
                    access_token: "".into(),
                    need_signup: true,
                };

                Json(response).into_response()
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

async fn get_github_access_token(
    database: Extension<Arc<Client>>,
    Json(body): Json<GithubAccessTokenRequest>,
) -> impl IntoResponse {
    let _user_service = UserService::new(database.clone());
    let auth_service = AuthService::new(database);

    let access_token = auth_service.get_github_access_token(&body.code).await;

    match access_token {
        Some(access_token) => {
            let response = GithubAccessTokenResponse { access_token };

            Json(response).into_response()
        }
        None => (StatusCode::BAD_REQUEST).into_response(),
    }
}
