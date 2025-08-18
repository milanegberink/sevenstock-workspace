use crate::{
    error::{Error, Result},
    middleware::mw_auth::CtxW,
};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use lib_core::model::ModelManager;
use redis::AsyncCommands;

pub async fn mw_rate_limiter(
    State(mm): State<ModelManager>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    if let Some(ctx) = req.extensions().get::<CtxW>() {
        let redis_key = format!("rate_limit:{}", ctx.0.sub());
        let x: u16 = mm.redis().get(redis_key).await.unwrap();
    }

    Ok(next.run(req).await)
}
