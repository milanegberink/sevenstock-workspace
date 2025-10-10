use crate::{error::Result, services::Services};

use axum::Json;
use axum::extract::State;
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use lib_grpc::{LoginRequest, Request};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_json::{Value, json};

pub async fn api_login_handler(
    State(services): State<Services>,
    cookies: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<Value>)> {
    let LoginPayload { email, password } = payload;

    let req = LoginRequest {
        email,
        pwd_clear: password.expose_secret().into(),
    };

    let res = services.auth().login(Request::new(req)).await?.into_inner();

    let cookie = Cookie::build(("refresh_token", res.refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let cookies = cookies.add(cookie);

    let response = json!({
        "access_token": res.access_token,
    });

    Ok((cookies, Json(response)))
}

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: SecretString,
}
