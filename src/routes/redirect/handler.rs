use std::sync::Arc;
use std::{collections::HashMap, error::Error};

use aws_sdk_dynamodb::Client;
use axum::extract::Query;
use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Extension, Json, Router,
};

use crate::extensions::{CurrentUser, DynamoClient};

use crate::{
    middlewares::auth_middleware,
    routes::{auth, user},
};

use super::dto::GithubRedirectCodeRequest;

pub(crate) async fn router() -> Router {
    Router::new().route("/github", get(get_github_code))
}

async fn get_github_code(Query(query): Query<GithubRedirectCodeRequest>) -> impl IntoResponse {
    let base_url = query
        .redirect_url
        .unwrap_or_else(|| "https://tokkitang.com/redirect/github".to_string());
    let code = query.code;

    let url = format!("{base_url}?code={code}",);

    Redirect::permanent(url.as_str()).into_response()
}
