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
    models::{project, Entity, InsertUser, Note, Project, Team, TeamUser, TeamUserAuthority, User},
    routes::{auth::AuthService, project::ProjectService, team::TeamService, user::UserService},
    utils::{generate_uuid, hash_password, AllError},
};

use super::{
    dto::{
        CreateEntityRequest, CreateEntityResponse, GetEntityItem, GetEntityResponse,
        UpdateEntityRequest, UpdateEntityResponse,
    },
    EntityService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/", post(create_entity))
        .route("/:entity_id", put(update_entity))
        .route("/:entity_id", get(get_entity))
        .route("/:entity_id", delete(delete_entity));

    app
}

async fn create_entity(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Json(body): Json<CreateEntityRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let entity_service = EntityService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = CreateEntityResponse {
        success: false,
        entity_id: "".into(),
    };

    let project = match project_service
        .get_project_by_id(body.project_id.clone())
        .await
    {
        Ok(project) => project,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    let team_id = project.team_id.clone();

    match team_service
        .find_team_user_by_team_and_user_id(team_id.clone(), user.id.clone())
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin | TeamUserAuthority::Write => {
                println!("# 권한 허용: OWNER OR ADMIN OR WRITE");
            }
            _ => {
                println!("# 권한 부족: NEED WRITE");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Entity {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: body.project_id,
        physical_name: body.physical_name,
        logical_name: body.logical_name,
        comment: body.comment,
        columns: body.columns,
        x: body.x,
        y: body.y,
    };

    match entity_service.create_entity(data).await {
        Ok(entity_id) => {
            response.entity_id = entity_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn update_entity(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(entity_id): Path<String>,
    Json(body): Json<UpdateEntityRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let entity_service = EntityService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = UpdateEntityResponse { success: false };

    let entity = match entity_service.get_entity_by_id(&entity_id).await {
        Ok(entity) => entity,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 엔티티 없음");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    let project_id = &entity.project_id;

    let project = match project_service.get_project_by_id(project_id).await {
        Ok(project) => project,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    let team_id = &project.team_id;

    match team_service
        .find_team_user_by_team_and_user_id(team_id, &user.id)
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin | TeamUserAuthority::Write => {
                println!("# 권한 허용: OWNER OR ADMIN OR WRITE");
            }
            _ => {
                println!("# 권한 부족: NEED WRITE");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Entity {
        id: entity_id,
        project_id: entity.project_id,
        physical_name: body.physical_name,
        logical_name: body.logical_name,
        comment: body.comment,
        columns: body.columns,
        x: body.x,
        y: body.y,
    };

    match entity_service.create_entity(data).await {
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

async fn delete_entity(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(entity_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let entity_service = EntityService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = UpdateEntityResponse { success: false };

    let entity = match entity_service.get_entity_by_id(&entity_id).await {
        Ok(entity) => entity,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    let project = match project_service.get_project_by_id(&entity.project_id).await {
        Ok(project) => project,
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    };

    match team_service
        .find_team_user_by_team_and_user_id(&project.team_id, &user.id)
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin | TeamUserAuthority::Write => {
                println!("# 권한 허용: OWNER OR ADMIN OR WRITE");
            }
            _ => {
                println!("# 권한 부족: NEED WRITE");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    match entity_service.delete_entity(&entity_id).await {
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

async fn get_entity(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(entity_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let project_service = ProjectService::new(database.clone());
    let entity_service = EntityService::new(database.clone());
    let team_service = TeamService::new(database.clone());

    let (entity_data, project_id) = match entity_service.get_entity_by_id(entity_id).await {
        Ok(entity) => (
            GetEntityItem {
                id: entity.id,
                physical_name: entity.physical_name,
                logical_name: entity.logical_name,
                comment: entity.comment,
                columns: entity.columns,
                x: entity.x,
                y: entity.y,
            },
            entity.project_id,
        ),
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 엔티티 없음");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    let team_id = match project_service.get_project_by_id(project_id).await {
        Ok(project) => project.team_id,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::FORBIDDEN).into_response();
            } else {
                println!("error: {error:?}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    match team_service
        .find_team_user_by_team_and_user_id(team_id, user.id)
        .await
    {
        Ok(_) => {
            println!("# 권한 허용");
        }
        Err(error) => {
            println!("error: {error:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let response = GetEntityResponse { data: entity_data };

    Json(response).into_response()
}
