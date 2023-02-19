use std::sync::Arc;
use std::{collections::HashMap, error::Error};

use aws_sdk_dynamodb::Client;
use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};

use crate::extensions::{CurrentUser, DynamoClient, S3Client};

use crate::middlewares::auth_middleware;
use crate::routes::{auth, redirect, team, user, utils};

pub(crate) async fn router() -> Router {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health)) 
        .nest("/api/utils", utils::router().await)
        .nest("/api/user", user::router().await)
        .nest("/api/auth", auth::router().await)
        .nest("/api/redirect", redirect::router().await)
        .nest("/api/team", team::router().await)
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(DynamoClient::get_client().await))
        .layer(Extension(S3Client::get_client().await));

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dto::health_response::HealthReponse;

async fn health(current_user: Extension<CurrentUser>) -> impl IntoResponse {
    let server_ok = true;
    let authorized = current_user.authorized;

    Json(HealthReponse {
        server_ok,
        authorized,
    })
    .into_response()
}
