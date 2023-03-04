use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use futures::future::join_all;

use crate::{
    extensions::CurrentUser,
    middlewares::auth,
    models::{InsertUser, Team, TeamUser, TeamUserAuthority, User},
    routes::{auth::AuthService, user::UserService},
    utils::{generate_uuid, hash_password},
};

use super::ProjectService;

pub async fn router() -> Router {
    let app = Router::new();

    app
}
