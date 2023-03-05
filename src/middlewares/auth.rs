use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use url::Url;

use crate::{extensions::CurrentUser, routes::user::UserService, utils::jwt};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|e| e.to_owned());

    let auth_query = {
        let uri = "http://fake.host".to_owned() + req.uri().to_string().as_str();
        let parsed_url = Url::parse(uri.as_str()).ok();
        parsed_url.map(|e| {
            e.query_pairs()
                .into_owned()
                .find(|(key, _)| *key == "AUTHORIZATION")
                .map(|(_, value)| value)
        })
    }
    .flatten();

    let auth_header = auth_header.or(auth_query);

    let mut current_user = CurrentUser::default();

    if let Some(auth_header) = auth_header {
        println!(">> Authorization: {auth_header}");
        let auth_header = auth_header.replace("Bearer ", "");

        let user_id = jwt::verify(auth_header.to_owned());

        if let Some(user_id) = user_id {
            println!(">> Authorization: JWT verify success");
            let client = req.extensions().get::<Arc<Client>>().unwrap();
            let user_service = UserService::new(Extension(client.to_owned()));

            if let Ok(Some(user)) = user_service.find_by_id(user_id).await {
                println!(">> Authorization: complete");
                current_user = CurrentUser {
                    user: Some(user),
                    authorized: true,
                };
            } else {
                println!(">> Authorization: user find failed");
            }
        } else {
            println!(">> Authorization: JWT verify failed");
        }
    }

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
