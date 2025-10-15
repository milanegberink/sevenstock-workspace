use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use lib_auth::{oauth2::OAuthQuery, token::Token};
use lib_core::model::ModelManager;
use serde::Deserialize;

use crate::Result;

pub async fn oauth2_authorize_handler(
    State(mm): State<ModelManager>,
    Query(req): Query<OAuthQuery>,
) -> impl IntoResponse {
    let token = Token::oauth2()
        .oauth2_params(req)
        .build_async()
        .await
        .unwrap();

    let login_url = format!("/login?token={}", token);

    Redirect::to(&login_url)
}
