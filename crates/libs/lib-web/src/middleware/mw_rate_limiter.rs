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
    let x = format!("rate_limit:{}");
    mm.redis().get(x);
    if let Some(ctx) = req.extensions().get::<CtxW>() {
        println!("Got ctx: {:?}", ctx);
    } else {
        println!("No ctx found!");
    }

    Ok(next.run(req).await)
}
