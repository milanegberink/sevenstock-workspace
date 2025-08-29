use aws_config::BehaviorVersion;
use aws_lambda_events::event::secretsmanager::SecretsManagerSecretRotationEvent;
use aws_sdk_secretsmanager::{config::IntoShared, Client};
use lambda_runtime::{tracing, Error, LambdaEvent};
use lib_auth::token::rotating_keys::SecretManager;

pub(crate) async fn function_handler(
    event: LambdaEvent<SecretsManagerSecretRotationEvent>,
) -> Result<(), Error> {
    let payload = event.payload;

    let config = aws_config::defaults(BehaviorVersion::latest());

    let secrets_client = aws_sdk_secretsmanager::Client::new(&config);

    let mut secret_manager = get_secret_manager(&client, payload.secret_id).await?;

    secret_manager.rotate();

    tracing::info!("Payload: {:?}", payload);

    Ok(())
}

async fn get_secret_manager(client: &Client, secret_id: &str) -> Result<SecretManager, Error> {
    let res = client
        .get_secret_value()
        .secret_id(secret_id)
        .send()
        .await?;

    let secret = res.secret_string()?;

    let sm: SecretManager = secret.parse()?;

    Ok(sm)
}

async fn generate_new_secret(client: &Client, secret_id: &str) {
    client
        .update_secret()
        .name(secret_id)
        .secret_string("placeholder")
        .send()
        .await?
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_event_handler() {
        let event = LambdaEvent::new(
            SecretsManagerSecretRotationEvent::default(),
            Context::default(),
        );
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
    }
}
