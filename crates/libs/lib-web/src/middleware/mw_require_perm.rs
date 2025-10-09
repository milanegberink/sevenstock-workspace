use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use uuid::Uuid;

use crate::{Result, middleware::mw_auth::CtxW, permission::Permission, services::Services};

pub async fn mw_require_permissions<const N: usize>(
    permissions: [Permission; N],
    State(mm): State<Services>,
    ctx: CtxW,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let ctx = ctx.0;

    Ok(next.run(req).await)
}
