use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use lib_auth::{
    oauth2::OAuthQuery,
    token::{AccessToken, TokenType},
};
use lib_core::model::ModelManager;
use serde::Deserialize;
use uuid::Uuid;

use crate::Result;

pub async fn oauth2_authorize_handler(
    State(mm): State<ModelManager>,
    Query(req): Query<OAuthQuery>,
) -> impl IntoResponse {
    let login_url = format!("/login?token={}", "X");

    Redirect::to(&login_url)
}
