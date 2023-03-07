use aws_sdk_ses::{model::Destination, Client};

use super::AllError;

const SENDER_EMAIL: &str = "service@tokkitang.com";

pub async fn send_email(target: &str, title: &str, content: &str) -> Result<(), AllError> {
    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);

    let result = client
        .send_email()
        .source(SENDER_EMAIL)
        .destination(Destination::builder().to_addresses(target).build())
        .message(
            aws_sdk_ses::model::Message::builder()
                .body(
                    aws_sdk_ses::model::Body::builder()
                        .html(aws_sdk_ses::model::Content::builder().data(content).build())
                        .build(),
                )
                .subject(aws_sdk_ses::model::Content::builder().data(title).build())
                .build(),
        )
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("Email sent successfully");
            Ok(())
        }
        Err(e) => {
            println!("Error sending email: {e:?}",);
            Err(AllError::AWSError(format!("{e:?}")))
        }
    }
}
