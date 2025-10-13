use crate::{
    error::{Error, Result},
    services::Services,
};

use axum::{Form, Json, extract::State};
use axum_extra::{
    extract::{
        CookieJar,
        cookie::{Cookie, SameSite},
    },
    headers::authorization::Bearer,
};
use lib_grpc::{RefreshTokenRequest, Request};
use secrecy::SecretString;
use serde_json::{Value, json};

struct ExchangeForm {
    grant_type: String,
    refresh_token: SecretString,
    client_id: String,
    client_secret: SecretString,
}

struct ExchangeResponse {
    access_token: String,
    token_type: Bearer,
    expires_in: u64,
    refresh_token: String,
    id_token: String,
}

pub async fn exchange_refresh(
    State(services): State<Services>,
    Form(form): Form<ExchangeForm>,
) -> Result {
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
