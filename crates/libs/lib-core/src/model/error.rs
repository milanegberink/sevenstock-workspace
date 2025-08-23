pub type Result<T> = core::result::Result<T, Error>;
use aws_sdk_secretsmanager::{error::SdkError, operation::get_secret_value::GetSecretValueError};
use lib_auth::{pwd, token};

use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Entity not found: {entity} ({id})")]
    EntityNotFound { entity: &'static str, id: Uuid },

    #[error("Failed to create modelmanager")]
    CantCreateModelManagerProvider(String),

    // -- Modules
    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),

    // -- Externals
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
}
