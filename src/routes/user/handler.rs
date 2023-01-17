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
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{SignupRequest, SignupResponse},
    UserService,
};

pub async fn router() -> Router {
    let app = Router::new().route("/signup", post(signup));

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
