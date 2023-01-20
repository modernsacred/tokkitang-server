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
    routes::user::UserService,
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{LoginRequest, LoginResponse},
    AuthService,
};

pub async fn router() -> Router {
    let app = Router::new().route("/login", post(login));

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

                let hashed_password = hash_password(password, &salt);

                if hashed_password == user.password {
                    response.success = true;

                    let user_id = user.id;
                    let access_token = auth_service.get_access_token(user_id);

                    response.access_token = access_token;
                }
            }
        }
        Err(error) => {
            println!("error: {:?}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    Json(response).into_response()
}
