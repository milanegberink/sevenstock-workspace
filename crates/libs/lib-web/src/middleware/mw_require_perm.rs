use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use lib_core::model::permission::{Action, Resource};
use uuid::Uuid;

// use crate::{Result, middleware::mw_auth::CtxW, permission::Permission, services::Services};

// pub async fn mw_require_permission(
//     permissions: &[(Resource, Action)],
//     State(mm): State<Services>,
//     ctx: CtxW,
//     req: Request<Body>,
//     next: Next,
// ) -> Result<Response> {
//     let active_permissions = ctx.0.permissions();

//     permissions.Ok(next.run(req).await)
// }
