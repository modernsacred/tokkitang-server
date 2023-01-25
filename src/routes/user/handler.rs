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
    routes::auth::AuthService,
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{SignupGithubRequest, SignupRequest, SignupResponse},
    UserService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/signup", post(signup))
        .route("/signup/github", post(signup_github));

    app
}

async fn signup(
    database: Extension<Arc<Client>>,
    Json(body): Json<SignupRequest>,
) -> impl IntoResponse {
    let service = UserService::new(database);
    let mut response = SignupResponse {
        email_duplicate: false,
        user_id: "".into(),
    };

    match service.exists_email(body.email.clone()).await {
        Ok(exists) => {
            if exists {
                response.email_duplicate = true;
                return (StatusCode::BAD_REQUEST, Json(response)).into_response();
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let email = body.email;
    let nickname = body.nickname;
    let original_password = body.password;
    let password_salt = generate_uuid();
    let hashed_password = hash_password(original_password, &password_salt);

    let user_data = User {
        id: uuid::Uuid::new_v4().to_string(),
        email,
        password: hashed_password,
        nickname,
        password_salt,
        github_id: None,
    };

    match service.create_user(user_data).await {
        Ok(user_id) => {
            response.user_id = user_id;
            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn signup_github(
    database: Extension<Arc<Client>>,
    Json(body): Json<SignupGithubRequest>,
) -> impl IntoResponse {
    let auth_service = AuthService::new(database.clone());
    let service = UserService::new(database);
    let mut response = SignupResponse {
        email_duplicate: false,
        user_id: "".into(),
    };

    match service.exists_email(body.email.clone()).await {
        Ok(exists) => {
            if exists {
                response.email_duplicate = true;
                return (StatusCode::BAD_REQUEST, Json(response)).into_response();
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let email = body.email;
    let nickname = body.nickname;
    let original_password = "github signup".into();
    let password_salt = generate_uuid();
    let hashed_password = hash_password(original_password, &password_salt);

    let github_user = auth_service.get_github_user(body.access_token).await;

    let github_user = match github_user {
        Some(github_user) => github_user,
        None => {
            return (StatusCode::BAD_REQUEST).into_response();
        }
    };

    let user_data = User {
        id: uuid::Uuid::new_v4().to_string(),
        email,
        password: hashed_password,
        nickname,
        password_salt,
        github_id: Some(github_user.id.to_string()),
    };

    match service.create_user(user_data).await {
        Ok(user_id) => {
            response.user_id = user_id;
            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
