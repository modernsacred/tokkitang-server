use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Project, Team, TeamUser},
    utils::AllError,
};

pub struct NoteService {
    client: Extension<Arc<Client>>,
}

impl NoteService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }
}
