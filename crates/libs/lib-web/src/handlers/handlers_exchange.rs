use crate::error::{Error, Result};

use axum::{Json, extract::State};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use lib_auth::token::{TokenBuilder, TokenType};
use lib_core::model::ModelManager;
use serde::Serialize;

pub async fn exchange_refresh(
    State(_mm): State<ModelManager>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<RefreshExchangeResponse>)> {
    let refresh_token = cookies
        .get("refresh_token")
        .ok_or(Error::NoRefreshTokenFound)?
        .value();

    let claims = TokenType::Refresh.verify(refresh_token).await?;

    let sub = claims.sub();

    let refresh_token = TokenBuilder::refresh().sub(sub).build_async().await?;

    let access_token = TokenBuilder::access()
        .sub(sub)
        .ident("Milan refreshed")
        .email("me@milanegberink.com")
        .build_async()
        .await?;

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let res = RefreshExchangeResponse { access_token };

    Ok((cookies.add(refresh_cookie), Json(res)))
}

#[derive(Serialize)]
pub struct RefreshExchangeResponse {
    access_token: String,
}
