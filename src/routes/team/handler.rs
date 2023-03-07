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
    routes::{
        auth::AuthService,
        project::{
            dto::{GetProjectListItem, GetProjectListResponse},
            ProjectService,
        },
        user::UserService,
    },
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        CreateTeamRequest, CreateTeamResponse, GetTeamItem, GetTeamListItem, GetTeamListResponse,
        GetTeamResponse, GetTeamUserListItem, GetTeamUserListResponse, UpdateTeamRequest,
        UpdateTeamResponse,
    },
    TeamService,
};

pub async fn router() -> Router {
    Router::new()
        .route("/", post(create_team))
        .route("/:team_id", get(get_team))
        .route("/:team_id", put(update_team))
        .route("/:team_id", delete(delete_team))
        .route("/:team_id/user/list", get(get_team_user_list))
        .route("/:team_id/project/list", get(get_team_project_list))
        .route("/my/list", get(get_my_team_list))
}

async fn get_team(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let team_user = match team_service
        .find_team_user_by_team_and_user_id(&team_id, &user.id)
        .await
    {
        Ok(Some(team_user)) => team_user,
        Ok(None) => {
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let team = match team_service.get_team_by_id(&team_id).await {
        Ok(team) => GetTeamItem {
            id: team.id,
            name: team.name,
            description: team.description,
            owner_id: team.owner_id,
            thumbnail_url: team.thumbnail_url,
            authority: team_user.authority,
        },
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let response = GetTeamResponse { data: team };

    Json(response).into_response()
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
        thumbnail_url: body.thumbnail_url,
        owner_id: user.id.clone(),
    };

    match team_service.create_team(team_data).await {
        Ok(team_id) => {
            response.team_id = team_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {error:?}");
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
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn update_team(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
    Json(body): Json<UpdateTeamRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());

    let mut response = UpdateTeamResponse { success: false };

    let old_team = match team_service.get_team_by_id(&team_id).await {
        Ok(team) => team,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    if old_team.owner_id != user.id {
        return (StatusCode::FORBIDDEN).into_response();
    }

    let team_data = Team {
        id: team_id,
        name: body.name,
        description: body.description,
        thumbnail_url: body.thumbnail_url,
        owner_id: user.id.clone(),
    };

    match team_service.create_team(team_data).await {
        Ok(_) => {
            response.success = true;
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn delete_team(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());

    let mut response = UpdateTeamResponse { success: false };

    let old_team = match team_service.get_team_by_id(&team_id).await {
        Ok(team) => team,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    if old_team.owner_id != user.id {
        return (StatusCode::FORBIDDEN).into_response();
    }

    match team_service.delete_team(&team_id).await {
        Ok(_) => {
            response.success = true;
        }
        Err(error) => {
            println!("error: {error:?}");
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

    let team_user_list = match team_service.get_team_user_list_by_user_id(&user.id).await {
        Ok(team_user_list) => team_user_list,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let team_list = join_all(team_user_list.into_iter().map(|team_user| async {
        match team_service.get_team_by_id(team_user.team_id).await {
            Ok(team) => Some(team),
            Err(_) => None,
        }
    }))
    .await;

    let team_list = team_list
        .into_iter()
        .filter_map(|e| match e {
            Some(team) => Some(GetTeamListItem {
                id: team.id,
                name: team.name,
                description: team.description,
                owner_id: team.owner_id,
                thumbnail_url: team.thumbnail_url,
            }),
            None => None,
        })
        .collect::<Vec<_>>();

    let response = GetTeamListResponse { list: team_list };

    Json(response).into_response()
}

async fn get_team_user_list(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    let _user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let user_service = UserService::new(database.clone());
    let team_service = TeamService::new(database.clone());

    let team_user_list = match team_service.get_team_user_list_by_team_id(&team_id).await {
        Ok(team_user_list) => team_user_list,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let user_list = join_all(team_user_list.into_iter().map(|team_user| async {
        match user_service.find_by_id(team_user.user_id).await {
            Ok(Some(user)) => Some(GetTeamUserListItem {
                id: user.id,
                nickname: user.nickname,
                email: user.email,
                thumbnail_url: user.thumbnail_url,
                authority: team_user.authority,
            }),
            _ => None,
        }
    }))
    .await;

    let user_list = user_list.into_iter().flatten().collect::<Vec<_>>();

    let response = GetTeamUserListResponse { list: user_list };

    Json(response).into_response()
}

async fn get_team_project_list(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    match team_service
        .find_team_user_by_team_and_user_id(&team_id, &user.id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return (StatusCode::FORBIDDEN).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }

    let project_list = match project_service.get_project_list_by_team_id(&team_id).await {
        Ok(team_user_list) => team_user_list,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let project_list = project_list
        .into_iter()
        .map(|e| GetProjectListItem {
            id: e.id,
            name: e.name,
            description: e.description,
            thumbnail_url: e.thumbnail_url,
        })
        .collect::<Vec<_>>();

    let response = GetProjectListResponse { list: project_list };

    Json(response).into_response()
}
