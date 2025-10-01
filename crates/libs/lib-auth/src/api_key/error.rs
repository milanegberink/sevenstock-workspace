use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize, Debug, Error)]
pub enum Error {
    #[error("MEOW MEOW")]
    ApiKeyInvalid,
}
