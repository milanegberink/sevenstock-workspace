use aws_config::BehaviorVersion;
use aws_lambda_events::event::secretsmanager::SecretsManagerSecretRotationEvent;
use aws_sdk_secretsmanager::{config::IntoShared, Client};
use jose_jwk::Jwk;
use lambda_runtime::{tracing, Error, LambdaEvent};
use lib_auth::token::jwks::{generate_key_pair, generate_private_jwk, Jwk, JwkSet};

pub(crate) async fn function_handler(
    event: LambdaEvent<SecretsManagerSecretRotationEvent>,
) -> Result<(), Error> {
    let payload = event.payload;

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let private_jwk = generate_private_jwk();

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
