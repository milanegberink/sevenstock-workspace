use crate::error::{Error, Result};
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{HeaderValue, Request, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{
        Authorization,
        authorization::{Bearer, Credentials},
    },
};
use lib_core::{ctx::Ctx, model::ModelManager};
use thiserror::Error;
use tracing::debug;
use uuid::Uuid;

pub struct ApiKey(pub String);

impl Credentials for ApiKey {
    const SCHEME: &'static str = "ApiKey";

    fn encode(&self) -> HeaderValue {
        let value = format!("{} {}", Self::SCHEME, self.0);
        HeaderValue::from_str(&value).expect("Invalid header value")
    }

    fn decode(value: &HeaderValue) -> Option<Self> {
        let s = value.to_str().ok()?;
        s.strip_prefix(&(Self::SCHEME.to_string() + " "))
            .map(|key| ApiKey(key.to_string()))
    }
}

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

    debug!("{}", token_hdr.token());

    let ctx_ext_result = ctx_resolve(mm, token_hdr.token()).await;

    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(_mm: ModelManager, _token: &str) -> CtxExtResult {
    Ctx::new(Uuid::now_v7())
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
