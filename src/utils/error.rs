#[derive(Debug, thiserror::Error)]
pub enum AllError {
    #[error("aws error: {0}")]
    AWSError(String),
}
