use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("MEOW MEOW")]
    InvalidToken,

    #[error("Failed to get secret string from retrieved secret")]
    SecretStringFromSecret,

    #[error("This is actually really fucking bad, if this happens youre fucked.")]
    PrimaryKeyNotFound,

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid token")]
    InvalidJwkSet,

    #[error("Config was already initialized")]
    AlreadyInitialized,

    #[error(transparent)]
    TokenEncodeFail(#[from] jsonwebtoken::errors::Error),
}
