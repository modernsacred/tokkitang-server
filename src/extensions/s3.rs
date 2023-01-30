use std::sync::Arc;

use aws_sdk_s3::Client;

pub struct S3Client {}

impl S3Client {
    pub async fn get_client() -> Arc<Client> {
        let config = aws_config::from_env().load().await;
        let client = Client::new(&config);

        Arc::new(client)
    }
}
