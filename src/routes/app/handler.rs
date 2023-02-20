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
// use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::extensions::{CurrentUser, DynamoClient, S3Client};

use crate::middlewares::auth_middleware;
use crate::routes::{auth, redirect, team, user, utils};

pub(crate) async fn router() -> Router {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    //let cors = CorsLayer::permissive();
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/utils", utils::router().await)
        .nest("/user", user::router().await)
        .nest("/auth", auth::router().await)
        .nest("/redirect", redirect::router().await)
        .nest("/team", team::router().await)
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(DynamoClient::get_client().await))
        .layer(Extension(S3Client::get_client().await))
        //.layer(cors)
        .layer(trace);

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
