use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    extensions::CurrentUser,
    middlewares::auth,
    models::{InsertUser, Team, TeamUser, TeamUserAuthority, User},
    routes::auth::AuthService,
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{CreateTeamRequest, CreateTeamResponse},
    TeamService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/", post(create_team))
        .route("/my/list", post(create_team));

    app
}

async fn create_team(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Json(body): Json<CreateTeamRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());

    let mut response = CreateTeamResponse {
        success: false,
        team_id: "".into(),
    };

    let team_data = Team {
        id: uuid::Uuid::new_v4().to_string(),
        name: body.name,
        description: body.description,
        thumbnail_url: None,
        owner_id: user.id.clone(),
    };

    match team_service.create_team(team_data).await {
        Ok(team_id) => {
            response.team_id = team_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let team_user_data = TeamUser {
        team_id: response.team_id.clone(),
        user_id: user.id,
        authority: TeamUserAuthority::Owner,
    };

    match team_service.create_team_user(team_user_data).await {
        Ok(()) => {}
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn get_my_team_list(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());

    let team_user_list = match team_service.get_team_user_list_by_user_id(user.id).await {
        Ok(team_user_list) => team_user_list,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let mut response = CreateTeamResponse {
        success: false,
        team_id: "".into(),
    };

    Json(response).into_response()
}
