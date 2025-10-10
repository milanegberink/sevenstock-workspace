use crate::{
    error::{Error, Result},
    services::Services,
};

use axum::{Json, extract::State};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use lib_grpc::{RefreshTokenRequest, Request};
use serde_json::{Value, json};

pub async fn exchange_refresh(
    State(services): State<Services>,
    cookies: CookieJar,
) -> Result<(CookieJar, Json<Value>)> {
    let refresh_token: String = cookies
        .get("refresh_token")
        .ok_or(Error::NoRefreshTokenFound)?
        .value()
        .into();

    let request = Request::new(RefreshTokenRequest { refresh_token });

    let response = services.auth().refresh_token(request).await?.into_inner();

    let refresh_cookie = Cookie::build(("refresh_token", response.refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let res = json!({ "access_token": response.access_token });

    Ok((cookies.add(refresh_cookie), Json(res)))
}
