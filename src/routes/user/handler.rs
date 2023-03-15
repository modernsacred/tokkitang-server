use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    extensions::CurrentUser,
    middlewares::auth,
    models::{InsertUser, User},
    routes::auth::AuthService,
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        GetEmailDuplicateRequest, GetEmailDuplicateResponse, MyInfoResponse, SignupGithubRequest,
        SignupRequest, SignupResponse,
    },
    UserService,
};

pub async fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/signup/github", post(signup_github))
        .route("/my/info", get(get_my_info))
        .route("/email/duplicate", get(get_email_duplicate))
}

async fn signup(
    database: Extension<Arc<Client>>,
    Json(body): Json<SignupRequest>,
) -> impl IntoResponse {
    let service = UserService::new(database.clone());
    let auth_service = AuthService::new(database);
    let mut response = SignupResponse {
        email_duplicate: false,
        access_token: "".into(),
        success: false,
    };

    match service.exists_email(&body.email).await {
        Ok(exists) => {
            if exists {
                response.email_duplicate = true;
                response.success = false;
                return (StatusCode::BAD_REQUEST, Json(response)).into_response();
            }
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let email = body.email;
    let nickname = body.nickname;
    let original_password = body.password;
    let password_salt = generate_uuid();
    let hashed_password = hash_password(&original_password, &password_salt);

    let user_data = User {
        id: uuid::Uuid::new_v4().to_string(),
        email,
        password: hashed_password,
        nickname,
        password_salt,
        thumbnail_url: None,
        github_id: None,
    };

    match service.create_user(user_data).await {
        Ok(user_id) => {
            let access_token = auth_service.get_access_token(user_id);

            response.access_token = access_token;
            response.success = true;
            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {error:?}");
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
        access_token: "".into(),
        success: false,
    };

    match service.exists_email(&body.email).await {
        Ok(exists) => {
            if exists {
                response.email_duplicate = true;
                response.success = false;
                return (StatusCode::BAD_REQUEST, Json(response)).into_response();
            }
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let email = body.email;
    let nickname = body.nickname;
    let original_password = "github signup".to_string();
    let password_salt = generate_uuid();
    let hashed_password = hash_password(&original_password, &password_salt);

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
        thumbnail_url: None,
    };

    match service.create_user(user_data).await {
        Ok(user_id) => {
            response.access_token = auth_service.get_access_token(user_id);
            response.success = true;
            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {error:?}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn get_my_info(
    current_user: Extension<CurrentUser>,
    _database: Extension<Arc<Client>>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let response = MyInfoResponse {
        id: user.id,
        nickname: user.nickname,
        email: user.email,
        thumbnail_url: user.thumbnail_url,
    };

    Json(response).into_response()
}

async fn get_email_duplicate(
    database: Extension<Arc<Client>>,
    Query(body): Query<GetEmailDuplicateRequest>,
) -> impl IntoResponse {
    let service = UserService::new(database.clone());

    match service.exists_email(&body.email).await {
        Ok(exists) => {
            if exists {
                let response = GetEmailDuplicateResponse { duplicate: true };

                return (Json(response)).into_response();
            } else {
                let response = GetEmailDuplicateResponse { duplicate: false };

                return (Json(response)).into_response();
            }
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }
}
