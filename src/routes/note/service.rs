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

    pub async fn get_note_by_id(&self, note_id: impl Into<String>) -> Result<Note, AllError> {
        match self
            .client
            .scan()
            .table_name(Note::NAME)
            .filter_expression("id = :note_id")
            .expression_attribute_values(":note_id", AttributeValue::S(note_id.into()))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| Note::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => return Err(AllError::AWSError(format!("{:?}", error))),
        }
    }

    pub async fn delete_note(&self, note_id: impl Into<String>) -> Result<(), AllError> {
        match self
            .client
            .delete_item()
            .table_name(Note::NAME)
            .key("id", AttributeValue::S(note_id.into()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }
}
