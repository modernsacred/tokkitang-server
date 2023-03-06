use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Entity, Note, Project, Team, TeamUser},
    utils::AllError,
};

pub struct EntityService {
    client: Extension<Arc<Client>>,
}

impl EntityService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create_entity(&self, data: Entity) -> Result<String, AllError> {
        let input = data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(Entity::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(data.id),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn get_entity_by_id(&self, entity_id: impl Into<String>) -> Result<Entity, AllError> {
        match self
            .client
            .scan()
            .table_name(Entity::NAME)
            .filter_expression("id = :entity_id")
            .expression_attribute_values(":entity_id", AttributeValue::S(entity_id.into()))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| Entity::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn delete_entity(&self, entity_id: impl Into<String>) -> Result<(), AllError> {
        match self
            .client
            .delete_item()
            .table_name(Entity::NAME)
            .key("id", AttributeValue::S(entity_id.into()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn get_entity_list_by_project_id(
        &self,
        project_id: impl Into<String>,
    ) -> Result<Vec<Entity>, AllError> {
        let mut list = vec![];
        let mut last_evaluated_key = None;

        let project_id = project_id.into();

        loop {
            match self
                .client
                .scan()
                .table_name(Entity::NAME)
                .filter_expression("project_id = :project_id")
                .expression_attribute_values(":project_id", AttributeValue::S(project_id.clone()))
                .set_exclusive_start_key(last_evaluated_key)
                .send()
                .await
            {
                Ok(data) => {
                    if let Some(items) = data.items() {
                        for item in items {
                            if let Some(team_user) = Entity::from_hashmap(item.to_owned()) {
                                list.push(team_user);
                            }
                        }
                    }

                    match data.last_evaluated_key() {
                        None => return Ok(list),
                        Some(key) => {
                            last_evaluated_key = Some(key.to_owned());
                            continue;
                        }
                    }
                }
                Err(error) => return Err(AllError::AWSError(format!("{error:?}"))),
            }
        }
    }
}
