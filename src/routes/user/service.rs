use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::models::{IndexEmailForUser, User};

pub struct UserService {
    client: Extension<Arc<Client>>,
}

impl UserService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn exists_email(&self, email: String) -> Result<bool, Box<dyn Error>> {
        let result = self
            .client
            .scan()
            .table_name(IndexEmailForUser::NAME)
            .filter_expression("email = :email")
            .expression_attribute_values(":email", AttributeValue::S(email))
            .send()
            .await?;

        Ok(result.items().map(|e| e.len()).unwrap_or(0) > 0)
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<User>, Box<dyn Error>> {
        let email_index_list = self
            .client
            .scan()
            .table_name(User::NAME)
            .filter_expression(format!("email = {}", email))
            .send()
            .await?;

        let user_id = email_index_list
            .items()
            .map(|e| e.get(0).map(|e| e.get("user_id")).flatten())
            .flatten();

        match user_id {
            Some(user_id) => {
                let user = self
                    .client
                    .get_item()
                    .table_name(User::NAME)
                    .key("id", user_id.to_owned())
                    .send()
                    .await?;

                Ok(User::from_hashmap(user.item()))
            }
            None => Ok(None),
        }
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<User>, Box<dyn Error>> {
        let user = self
            .client
            .get_item()
            .table_name(User::NAME)
            .key("id", AttributeValue::S(user_id))
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

        let index_email = IndexEmailForUser {
            email: user_data.email,
            user_id: user_data.id.clone(),
        };

        let input = index_email.to_hashmap();

        let _email_index = self
            .client
            .put_item()
            .table_name(IndexEmailForUser::NAME)
            .set_item(input)
            .send()
            .await?;

        Ok(user_data.id)
    }
}
