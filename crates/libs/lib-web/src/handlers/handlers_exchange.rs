use crate::{Error, Result};

use axum::extract::State;
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use lib_auth::{
    pwd::verify_password,
    token::{TokenBuilder, TokenType},
};
use lib_core::model::ModelManager;

pub async fn exchange_refresh(
    State(_mm): State<ModelManager>,
    cookies: CookieJar,
) -> Result<(CookieJar, RefreshExchangeResponse)> {
    let refresh_token = cookies
        .get("refresh_token")
        .ok_or(Error::NoRefreshTokenFound)?
        .value();

    let claims = TokenType::Refresh.verify(refresh_token).await?;

    let sub = claims.sub();

    let refresh_token = TokenBuilder::refresh().sub(sub).build_async().await?;

    let access_token = TokenBuilder::access()
        .sub(sub)
        .ident("X")
        .build_async()
        .await?;

    let refresh_cookie = Cookie::build(("refresh_token", "x"))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let res = RefreshExchangeResponse { access_token };

    Ok((cookies.add(refresh_cookie), res))
}

struct RefreshExchangeResponse {
    access_token: String,
}
