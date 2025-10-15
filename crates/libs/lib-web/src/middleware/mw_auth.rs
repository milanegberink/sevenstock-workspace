use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

use crate::error::{Error, Result};
use axum::http::HeaderValue;
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::{Bearer, Credentials},
    },
};
use lib_auth::token::{Claims, TokenType};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::permission::{Action, Permission, Permissions, Resource};
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(mm): State<ModelManager>,
    token_hdr: TypedHeader<Authorization<Bearer>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let token = token_hdr.token();

    debug!("{}", token);

    let ctx_ext_result = ctx_resolve(mm, token).await;

    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(_services: ModelManager, token: &str) -> CtxExtResult {
    let claims = TokenType::Access
        .verify(token)
        .await
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))?;

    let mut permissions: Permissions = HashMap::new();

    let Some(scope) = claims.scope() else {
        return Err(CtxExtError::TokenWrongFormat);
    };

    for token in scope.split_whitespace() {
        if let Some((resource_str, action_str)) = token.split_once(':') {
            let resource = Resource::from_str(resource_str)
                .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))?;

            let action = Action::from_str(action_str)
                .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))?;

            permissions
                .entry(resource)
                .or_insert_with(HashSet::new)
                .insert(action);
        }
    }

    let Some(sub) = claims.sub() else {
        return Err(CtxExtError::TokenWrongFormat);
    };

    Ctx::new(sub.clone())
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Debug, Error, Clone)]
pub enum CtxExtError {
    #[error("Token not in cookie")]
    TokenNotInCookie,
    #[error("Token not in cookie")]
    TokenWrongFormat,
    #[error("Token not in cookie")]
    UserNotFound,
    #[error("Token not in cookie")]
    ModelAccessError(String),
    #[error("Token not in cookie")]
    FailValidate,
    #[error("Token not in cookie")]
    CannotSetTokenCookie,
    #[error("Meow meow")]
    CtxNotInRequestExt,
    #[error("Token not in cookie: {0}")]
    CtxCreateFail(String),
}
