use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Team, TeamUser},
    utils::AllError,
};

pub struct ProjectService {
    client: Extension<Arc<Client>>,
}

impl ProjectService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }
}
