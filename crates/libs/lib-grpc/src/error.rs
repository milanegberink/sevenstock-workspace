use thiserror::Error;
use tonic::Status;
pub type Result<T> = core::result::Result<T, Status>;

#[derive(Error, Debug)]
pub enum Error {
    // -- Modules
    #[error(transparent)]
    Token(#[from] lib_auth::token::Error),

    #[error(transparent)]
    Pwd(#[from] lib_auth::pwd::Error),

    #[error(transparent)]
    Model(#[from] lib_core::model::Error),
}

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        Status::unauthenticated("x")
    }
}
