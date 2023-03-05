use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::models::User;

pub struct UserService {
    client: Extension<Arc<Client>>,
}

impl UserService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn exists_email(&self, email: impl Into<String>) -> Result<bool, Box<dyn Error>> {
        let result = self
            .client
            .scan()
            .table_name(User::NAME)
            .filter_expression("email = :email")
            .expression_attribute_values(":email", AttributeValue::S(email.into()))
            .send()
            .await?;

        Ok(result.items().map(|e| e.len()).unwrap_or(0) > 0)
    }

    pub async fn find_by_email(
        &self,
        email: impl Into<String>,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let scan_result = self
            .client
            .scan()
            .table_name(User::NAME)
            .filter_expression("email = :email")
            .expression_attribute_values(":email", AttributeValue::S(email.into()))
            .send()
            .await?;

        let user_list = scan_result.items();

        match user_list {
            Some(user_list) => Ok(User::from_hashmap(user_list.get(0))),
            None => Ok(None),
        }
    }

    pub async fn find_by_github_id(
        &self,
        github_id: impl Into<String>,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let scan_result = self
            .client
            .scan()
            .table_name(User::NAME)
            .filter_expression("github_id = :github_id")
            .expression_attribute_values(":github_id", AttributeValue::S(github_id.into()))
            .send()
            .await?;

        let user_list = scan_result.items();

        match user_list {
            Some(user_list) => Ok(User::from_hashmap(user_list.get(0))),
            None => Ok(None),
        }
    }

    pub async fn find_by_id(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let user = self
            .client
            .get_item()
            .table_name(User::NAME)
            .key("id", AttributeValue::S(user_id.into()))
            .send()
            .await?;

        Ok(User::from_hashmap(user.item()))
    }

    pub async fn create_user(&self, user_data: User) -> Result<String, Box<dyn Error>> {
        let input = user_data.to_hashmap();

        let _user = self
            .client
            .put_item()
            .table_name(User::NAME)
            .set_item(input)
            .send()
            .await?;

        Ok(user_data.id)
    }
}
