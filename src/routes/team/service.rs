use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Team, TeamUser},
    utils::AllError,
};

pub struct TeamService {
    client: Extension<Arc<Client>>,
}

impl TeamService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create_team(&self, team_data: Team) -> Result<String, AllError> {
        let input = team_data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(Team::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(team_data.id),
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }

    pub async fn create_team_user(&self, team_user: TeamUser) -> Result<(), AllError> {
        let input = team_user.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(TeamUser::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }

    pub async fn get_team_by_id(&self, team_id: String) -> Result<Team, AllError> {
        match self
            .client
            .scan()
            .table_name(Team::NAME)
            .filter_expression("id = :team_id")
            .expression_attribute_values(":team_id", AttributeValue::S(team_id))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| Team::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => return Err(AllError::AWSError(format!("{:?}", error))),
        }
    }

    pub async fn get_team_user_list_by_user_id(
        &self,
        user_id: String,
    ) -> Result<Vec<TeamUser>, AllError> {
        let mut list = vec![];
        let mut last_evaluated_key = None;

        loop {
            match self
                .client
                .scan()
                .table_name(TeamUser::NAME)
                .filter_expression("user_id = :user_id")
                .expression_attribute_values(":user_id", AttributeValue::S(user_id.clone()))
                .set_exclusive_start_key(last_evaluated_key)
                .send()
                .await
            {
                Ok(data) => {
                    if let Some(items) = data.items() {
                        for item in items {
                            if let Some(team_user) = TeamUser::from_hashmap(item.to_owned()) {
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
                Err(error) => return Err(AllError::AWSError(format!("{:?}", error))),
            }
        }
    }
}
