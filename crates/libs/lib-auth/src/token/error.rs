use aws_sdk_secretsmanager::{error::SdkError, operation::get_secret_value::GetSecretValueError};
use axum::body::HttpBody;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to encode token: {0}")]
    TokenEncodeFail(&'static str),

    #[error("MEOW MEOW")]
    InvalidToken,

    #[error("Failed to get secret string from retrieved secret")]
    SecretStringFromSecret,

    #[error("This is actually really fucking bad, if this happens youre fucked.")]
    PrimaryKeyNotFound,

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),

    #[error(transparent)]
    AwsNoSecretFoundWithId(#[from] SdkError<GetSecretValueError>),

    #[error("Invalid token")]
    InvalidJwkSet,
}
