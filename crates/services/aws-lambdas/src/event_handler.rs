use aws_config::BehaviorVersion;
use aws_lambda_events::event::secretsmanager::SecretsManagerSecretRotationEvent;
use aws_sdk_secretsmanager::{config::IntoShared, Client};
use lambda_runtime::{tracing, Error, LambdaEvent};
use lib_auth::token::jwks::generate_private_jwk;

pub(crate) async fn function_handler(
    event: LambdaEvent<SecretsManagerSecretRotationEvent>,
) -> Result<(), Error> {
    let payload = event.payload;

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let private_jwk = generate_private_jwk();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_event_handler() {}
}
