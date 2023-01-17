use std::sync::Arc;

use aws_sdk_dynamodb::Client;

pub struct DynamoClient {}

impl DynamoClient {
    pub async fn get_client() -> Arc<Client> {
        let config = aws_config::from_env().load().await;
        let client = Client::new(&config);

        Arc::new(client)
    }
}
