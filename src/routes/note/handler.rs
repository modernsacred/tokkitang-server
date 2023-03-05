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
    models::{InsertUser, Note, Project, Team, TeamUser, TeamUserAuthority, User},
    routes::{auth::AuthService, project::ProjectService, team::TeamService, user::UserService},
    utils::{generate_uuid, hash_password, AllError},
};

use super::{
    dto::{
        CreateNoteRequest, CreateNoteResponse, GetNoteItem, GetNoteResponse, UpdateNoteRequest,
        UpdateNoteResponse,
    },
    NoteService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/", post(create_note))
        .route("/:note_id", put(update_note))
        .route("/:note_id", get(get_note));

    app
}

async fn create_note(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Json(body): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let note_service = NoteService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = CreateNoteResponse {
        success: false,
        note_id: "".into(),
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
                println!("error: {:?}", error);
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
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Note {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: body.project_id.clone(),
        content: body.content.clone(),
        x: body.x,
        y: body.y,
    };

    match note_service.create_note(data).await {
        Ok(note_id) => {
            response.note_id = note_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}

async fn update_note(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(note_id): Path<String>,
    Json(body): Json<UpdateNoteRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let note_service = NoteService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = UpdateNoteResponse { success: false };

    let project_id = match note_service.get_note_by_id(note_id.clone()).await {
        Ok(note) => note.project_id,
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let team_id = match project_service.get_project_by_id(project_id.clone()).await {
        Ok(project) => project.team_id,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::FORBIDDEN).into_response();
            } else {
                println!("error: {:?}", error);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    match team_service
        .find_team_user_by_team_and_user_id(team_id, user.id)
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
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Note {
        id: note_id,
        project_id: project_id,
        content: body.content.clone(),
        x: body.x,
        y: body.y,
    };

    match note_service.create_note(data).await {
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

async fn get_note(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Path(note_id): Path<String>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let project_service = ProjectService::new(database.clone());
    let note_service = NoteService::new(database.clone());
    let team_service = TeamService::new(database.clone());

    let (note_data, project_id) = match note_service.get_note_by_id(note_id).await {
        Ok(note) => (
            GetNoteItem {
                id: note.id,
                content: note.content,
                x: note.x,
                y: note.y,
            },
            note.project_id,
        ),
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let team_id = match project_service.get_project_by_id(project_id).await {
        Ok(project) => project.team_id,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::FORBIDDEN).into_response();
            } else {
                println!("error: {:?}", error);
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
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let response = GetNoteResponse { data: note_data };

    Json(response).into_response()
}
