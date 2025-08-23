use crate::{
    error::{Error, Result},
    middleware::mw_auth::CtxW,
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use lib_core::model::ModelManager;
use redis::AsyncCommands;
use tracing::info;

pub async fn mw_rate_limiter(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let ctx = ctx.0;

    info!("{:?}", ctx);

    Ok(next.run(req).await)
}
