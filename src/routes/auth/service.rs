use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::Extension;
use epoch_timestamp::Epoch;
use std::error::Error;

use crate::{models::User, utils::jwt};

pub struct AuthService {
    _client: Extension<Arc<Client>>,
}

impl AuthService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { _client: client }
    }

    pub fn get_access_token(&self, user_id: String) -> String {
        let epoch = (Epoch::now() + Epoch::day(1)) as usize;

        jwt::sign(epoch, user_id)
    }
}
