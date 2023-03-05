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
    models::{InsertUser, Project, Team, TeamUser, TeamUserAuthority, User},
    routes::{auth::AuthService, team::TeamService, user::UserService},
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        CreateProjectRequest, CreateProjectResponse, GetProjectItem, GetProjectResponse,
        UpdateProjectRequest, UpdateProjectResponse,
    },
    ProjectService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/", post(create_project))
        .route("/:project_id", put(update_project))
        .route("/:project_id", delete(delete_project))
        .route("/:project_id", get(get_project));

    app
}

async fn create_project(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Json(body): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = CreateProjectResponse {
        success: false,
        project_id: "".into(),
    };

    match team_service
        .find_team_user_by_team_and_user_id(body.team_id.clone(), user.id.clone())
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin => {
                println!("# 권한 허용: OWNER OR ADMIN");
            }
            _ => {
                println!("# 권한 부족: NOT OWNER OR ADMIN");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: body.name,
        description: body.description,
        thumbnail_url: body.thumbnail_url,
        team_id: body.team_id,
    };

    match project_service.create_project(data).await {
        Ok(project_id) => {
            response.project_id = project_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn update_project(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(project_id): Path<String>,
    Json(body): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = UpdateProjectResponse { success: false };

    let old_team = match project_service.get_project_by_id(project_id.clone()).await {
        Ok(team) => team,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    match team_service
        .find_team_user_by_team_and_user_id(old_team.team_id.clone(), user.id.clone())
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin => {
                println!("# 권한 허용: OWNER OR ADMIN");
            }
            _ => {
                println!("# 권한 부족: NOT OWNER OR ADMIN");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Project {
        id: project_id,
        name: body.name,
        description: body.description,
        thumbnail_url: body.thumbnail_url,
        team_id: old_team.team_id,
    };

    match project_service.create_project(data).await {
        Ok(_) => {
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn delete_project(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = UpdateProjectResponse { success: false };

    let old_team = match project_service.get_project_by_id(project_id.clone()).await {
        Ok(team) => team,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    match team_service
        .find_team_user_by_team_and_user_id(old_team.team_id.clone(), user.id.clone())
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin => {
                println!("# 권한 허용: OWNER OR ADMIN");
            }
            _ => {
                println!("# 권한 부족: NOT OWNER OR ADMIN");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    match project_service.delete_project(project_id).await {
        Ok(_) => {
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn get_project(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let project_service = ProjectService::new(database.clone());
    let team_service = TeamService::new(database.clone());

    let (project_data, team_id) = match project_service.get_project_by_id(project_id).await {
        Ok(project) => (
            GetProjectItem {
                id: project.id,
                name: project.name,
                description: project.description,
                thumbnail_url: project.thumbnail_url,
            },
            project.team_id,
        ),
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match team_service
        .find_team_user_by_team_and_user_id(user.id.clone(), team_id)
        .await
    {
        Ok(_) => {
            println!("# 권한 허용");
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let response = GetProjectResponse { data: project_data };

    Json(response).into_response()
}
