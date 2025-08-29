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

    #[error(transparent)]
    Pkcs8Error(#[from] pkcs8::Error),

    #[error(transparent)]
    Pkcs8Error2(#[from] pkcs8::spki::Error),

    #[error("This is actually really fucking bad, if this happens youre fucked.")]
    PrimaryKeyNotFound,

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}
