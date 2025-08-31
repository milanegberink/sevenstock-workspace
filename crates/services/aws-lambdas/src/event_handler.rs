use aws_config::BehaviorVersion;
use aws_lambda_events::event::secretsmanager::SecretsManagerSecretRotationEvent;
use aws_sdk_secretsmanager::{config::IntoShared, Client};
use lambda_runtime::{tracing, Error, LambdaEvent};
use lib_auth::token::jwks::{Jwk, JwkSet, SecretConfig};

pub(crate) async fn function_handler(
    event: LambdaEvent<SecretsManagerSecretRotationEvent>,
) -> Result<(), Error> {
    let payload = event.payload;

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let secrets_client = aws_sdk_secretsmanager::Client::new(&config);

    let new_config = SecretConfig::new();

    let current_private_key: Jwk =
        serde_json::from_str(&get_secret(&secrets_client, &payload.secret_id).await).unwrap();

    let mut current_pubkeys: JwkSet = get_current_pubkeys().await;

    current_pubkeys.keys.push(current_private_key);

    secrets_client
        .update_secret()
        .secret_id(&payload.secret_id)
        .secret_string(new_config)
        .send()
        .await?;

    Ok(())
}

async fn get_current_pubkeys() -> JwkSet {
    // get from s3
    todo!()
}

async fn get_secret(client: &Client, secret_id: &str) -> String {
    client
        .get_secret_value()
        .secret_id(secret_id)
        .send()
        .await
        .unwrap()
        .secret_string
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_event_handler() {}
}
