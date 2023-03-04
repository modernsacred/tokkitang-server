use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Project, Team, TeamUser},
    utils::AllError,
};

pub struct ProjectService {
    client: Extension<Arc<Client>>,
}

impl ProjectService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create_project(&self, data: Project) -> Result<String, AllError> {
        let input = data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(Project::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(data.id),
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }

    pub async fn get_project_by_id(&self, project_id: String) -> Result<Project, AllError> {
        match self
            .client
            .scan()
            .table_name(Project::NAME)
            .filter_expression("id = :project_id")
            .expression_attribute_values(":project_id", AttributeValue::S(project_id))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| Project::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => return Err(AllError::AWSError(format!("{:?}", error))),
        }
    }
}
