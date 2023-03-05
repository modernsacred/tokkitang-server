use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Note, Project, Team, TeamUser},
    utils::AllError,
};

pub struct EntityService {
    client: Extension<Arc<Client>>,
}

impl EntityService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }
}
