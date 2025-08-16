use crate::error::Result;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use lib_core::ctx::Ctx;
use tracing::debug;

pub async fn mw_ctx_require(req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver() {
    todo!();
}
