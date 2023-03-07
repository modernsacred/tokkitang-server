#![allow(clippy::single_match)]
use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use futures::future::join_all;
use uuid::Uuid;

use crate::{
    extensions::CurrentUser,
    middlewares::auth,
    models::{InsertUser, Team, TeamInvite, TeamUser, TeamUserAuthority, User},
    routes::{
        auth::AuthService,
        project::{
            dto::{GetProjectListItem, GetProjectListResponse},
            ProjectService,
        },
        user::UserService,
    },
    utils::{generate_uuid, hash_password, send_email, AllError},
};

use super::{
    dto::{
        CreateTeamRequest, CreateTeamResponse, GetTeamItem, GetTeamListItem, GetTeamListResponse,
        GetTeamResponse, GetTeamUserListItem, GetTeamUserListResponse, InviteUserToTeamRequest,
        TransferOwnershipRequest, UpdateTeamRequest, UpdateTeamResponse,
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
        .route("/:team_id/user/invite", post(invite_user))
        .route("/:team_id/user/invite/:code/join", get(join_team))
        .route("/:team_id/ownership/transfer", post(transfer_ownership))
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

async fn invite_user(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
    Json(body): Json<InviteUserToTeamRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let user_service = UserService::new(database.clone());

    match team_service
        .find_team_user_by_team_and_user_id(&team_id, &user.id)
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            // Owner는 Admin/Write/Read로 초대 가능
            TeamUserAuthority::Owner => match body.authority {
                TeamUserAuthority::Owner => {
                    return (StatusCode::BAD_REQUEST).into_response();
                }
                _ => {}
            },
            // Admin은 Write/Read로 초대 가능
            TeamUserAuthority::Admin => match body.authority {
                TeamUserAuthority::Owner | TeamUserAuthority::Admin => {
                    return (StatusCode::BAD_REQUEST).into_response();
                }
                _ => {}
            },
            _ => {
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => return (StatusCode::FORBIDDEN).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let user_to_invite = match user_service.find_by_id(&body.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("# User Not Found");
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(error) => {
            println!("# User Found Error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if user_to_invite.id == user.id {
        println!("# User is same");
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let team_to_invite = match team_service.get_team_by_id(&team_id).await {
        Ok(team) => team,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# Team Not Found");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("# Team Found Error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };
    let team_name = team_to_invite.name;

    let code = match team_service
        .create_team_invite(TeamInvite {
            code: Uuid::new_v4().to_string(),
            team_id: team_id.clone(),
            user_id: user_to_invite.id.clone(),
            authority: body.authority,
        })
        .await
    {
        Ok(code) => code,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let title = format!("[{team_name}]팀에 초대합니다!");

    let nickname = user_to_invite.nickname;
    let host = "https://ksauqt5f5er2djql3atquzas4e0ofpla.lambda-url.ap-northeast-2.on.aws";
    let invite_url = format!("{host}/team/{team_id}/user/invite/{code}/join",);
    let content = format!(
        r#"안녕하세요 {nickname}님, {team_name}팀에 초대합니다!<br> 초대 링크: <a href="{invite_url}">{invite_url}</a>"#
    );

    match send_email(
        user_to_invite.email.as_str(),
        title.as_str(),
        content.as_str(),
    )
    .await
    {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn join_team(
    database: Extension<Arc<Client>>,
    Path((team_id, code)): Path<(String, String)>,
) -> impl IntoResponse {
    let team_service = TeamService::new(database.clone());

    let invite = match team_service.get_team_invite_by_code(&code).await {
        Ok(invite) => invite,
        Err(_) => {
            println!("# Invite Not Found");
            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    if invite.team_id != team_id {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let team_user_data = TeamUser {
        team_id: invite.team_id,
        user_id: invite.user_id,
        authority: invite.authority,
    };

    match team_service.create_team_user(team_user_data).await {
        Ok(()) => {
            if let Err(error) = team_service.delete_team_invite_by_code(&code).await {
                println!("# Invite Delete Error: {error:?}");
            }

            let url = format!("https://tokkitang.com");

            Redirect::permanent(url.as_str()).into_response()
        }
        Err(error) => {
            println!("error: {error:?}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn transfer_ownership(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(team_id): Path<String>,
    Json(body): Json<TransferOwnershipRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());

    let mut team = match team_service.get_team_by_id(&team_id).await {
        Ok(team) => team,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# Team Not Found");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("# Team Found Error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    match team_service
        .find_team_user_by_team_and_user_id(&team_id, &user.id)
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            // Owner만 사용가능
            TeamUserAuthority::Owner => {}
            _ => {
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => return (StatusCode::FORBIDDEN).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match team_service
        .create_team_user(TeamUser {
            team_id: team_id.clone(),
            user_id: user.id,
            authority: TeamUserAuthority::Admin,
        })
        .await
    {
        Ok(()) => {}
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    match team_service
        .create_team_user(TeamUser {
            team_id: team_id.clone(),
            user_id: body.user_id.clone(),
            authority: TeamUserAuthority::Owner,
        })
        .await
    {
        Ok(()) => {}
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    team.owner_id = body.user_id;

    match team_service.create_team(team).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(error) => {
            println!("error: {error:?}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
