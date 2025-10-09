use lib_auth::{pwd, token};
use thiserror::Error;
use tonic::Status;
pub type Result<T> = core::result::Result<T, Status>;

#[derive(Error, Debug)]
pub enum Error {
    // -- Modules
    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),

    #[error(transparent)]
    Model(#[from] lib_core::model::Error),
}

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::Token(_inner) => Status::unauthenticated("Unauthorized"),
            Error::Pwd(_inner) => Status::unauthenticated("Unauthorized"),
            Error::Model(_inner) => Status::unauthenticated("Unauthorized"),
        }
    }
}
