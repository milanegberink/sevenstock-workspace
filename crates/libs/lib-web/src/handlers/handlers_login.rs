use crate::error::Result;

use axum::Json;
use axum::extract::State;
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use lib_core::model::ModelManager;
use lib_grpc::{LoginRequest, Request};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<LoginResponse>)> {
    let LoginPayload {
        email,
        password: pwd_bytes,
    } = payload;

    let req = LoginRequest {
        email,
        password: pwd_bytes.expose_secret().into(),
    };

    let res = mm
        .auth()
        .login(Request::new(req))
        .await
        .unwrap()
        .into_inner();

    let cookie = Cookie::build(("refresh_token", res.refresh_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let cookies = cookies.add(cookie);

    let response = LoginResponse {
        access_token: res.access_token,
    };

    Ok((cookies, Json(response)))
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: SecretString,
}
