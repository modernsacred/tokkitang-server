use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::Extension;
use epoch_timestamp::Epoch;
use reqwest::header;
use std::error::Error;

pub struct UtilService {}

impl UtilService {
    pub fn new() -> Self {
        Self {}
    }
}
