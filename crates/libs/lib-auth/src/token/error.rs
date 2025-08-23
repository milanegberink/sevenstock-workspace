use aws_sdk_secretsmanager::{
    config::http::HttpResponse, error::SdkError, operation::create_secret::CreateSecretError,
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to encode token: {0}")]
    TokenEncodeFail(&'static str),
    #[error("MEOW MEOW")]
    InvalidToken,

    #[error(transparent)]
    SecretsError(#[from] SdkError<CreateSecretError, HttpResponse>),

    #[error(transparent)]
    Pkcs8Error(#[from] pkcs8::Error),
}
