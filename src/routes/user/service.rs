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
            .get_item()
            .table_name(IndexEmailForUser::NAME)
            .key("email", AttributeValue::S(email))
            .send()
            .await?;

        match result.item() {
            Some(item) => {
                result.item().unwrap().get("email").unwrap();
            }
            None => Ok(false),
        }
    }

    pub async fn find_by_email(&self, email: String) -> Result<Option<User>, Box<dyn Error>> {
        let user = self.client.collection::<User>(User::NAME);

        let filter = doc! {"email": email};
        let result = user.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, user_id: String) -> Result<Option<User>, Box<dyn Error>> {
        let user = self.client.collection::<User>(User::NAME);
        let user_id = ObjectId::from_str(user_id.as_str())?;

        let filter = doc! {"_id": user_id };
        let result = user.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn create_user(&self, user_data: User) -> Result<String, Box<dyn Error>> {
        let user = self.client.put_item().table_name(User::NAME);
        let result = user.insert_one(user_data, None).await?;
        let user_id = result.inserted_id.as_object_id().unwrap();
        let user_id = user_id.to_hex();

        Ok(user_id)
    }
}
