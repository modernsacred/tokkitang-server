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
    let app = Router::new()
        .route("/github/code", get(get_github_code))
        .route("/github/access-token", get(get_github_access_token));

    app
}

async fn get_github_code(Query(query): Query<GithubRedirectCodeRequest>) -> impl IntoResponse {
    let url = format!(
        "https://tokkitang.com/redirect/github-login?code={}",
        query.code
    );

    Redirect::permanent(url.as_str()).into_response()
}

async fn get_github_access_token(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    println!("{:?}", params);

    format!("{:?}", params).into_response()
}
