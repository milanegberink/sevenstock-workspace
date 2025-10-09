use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib_auth::{pwd, token};
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

use crate::middleware;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("User has no password")]
    LoginFailUserHasNoPwd { user_id: Uuid },

    #[error(transparent)]
    CtxExt(#[from] middleware::mw_auth::CtxExtError),

    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),

    #[error("No token found in cookie header")]
    NoRefreshTokenFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let response = StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();

        error!("{}", self);

        response
    }
}
