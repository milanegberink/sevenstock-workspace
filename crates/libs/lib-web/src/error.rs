use aws_sdk_secretsmanager::{
    error::SdkError,
    operation::{create_secret::CreateSecretError, get_secret_value::GetSecretValueError},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib_auth::{pwd, token};
use lib_core::model;
use thiserror::Error;
use tracing::error;

use crate::middleware;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CtxExt(#[from] middleware::mw_auth::CtxExtError),

    #[error(transparent)]
    Model(#[from] model::Error),

    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),

    #[error(transparent)]
    GetSecretError(#[from] SdkError<GetSecretValueError>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response = StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();

        error!("{}", self);

        response
    }
}
