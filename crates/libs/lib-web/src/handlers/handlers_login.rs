use crate::error::Result;

use axum::Json;
use axum::extract::State;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use lib_auth::pwd::verify_password;
use lib_auth::token::TokenBuilder;
use lib_core::model::ModelManager;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn api_login_handler(
    State(_mm): State<ModelManager>,
    cookies: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<LoginResponse>)> {
    let LoginPayload {
        email,
        password: pwd_clear,
    } = payload;

    verify_password("$argon2id$v=19$m=19456,t=2,p=1$CIqj9N/UsqaeW4qt6Y4dGg$PkGeH8Nj6NMO8oe1R8/WKEr14b8IL9nvYUEjVXcG8Yw".to_string(), pwd_clear)?;

    let uuid = Uuid::now_v7();

    let access_token = TokenBuilder::access()
        .sub(&uuid)
        .email(email)
        .ident("Milan")
        .build_async()
        .await?;

    let refresh_token = TokenBuilder::refresh().sub(&uuid).build_async().await?;

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let cookies = cookies.add(cookie);

    let response = LoginResponse { access_token };

    Ok((cookies, Json(response)))
}

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: SecretString,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
}
