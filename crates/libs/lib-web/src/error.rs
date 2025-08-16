use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib_auth::{pwd, token};
use lib_core::model;
use thiserror::Error;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Model(#[from] model::Error),

    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response = StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();

        error!("{}", self);

        response
    }
}
