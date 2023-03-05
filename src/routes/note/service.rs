use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Note, Project, Team, TeamUser},
    utils::AllError,
};

pub struct NoteService {
    client: Extension<Arc<Client>>,
}

impl NoteService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create_note(&self, data: Note) -> Result<String, AllError> {
        let input = data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(Note::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(data.id),
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }
}
