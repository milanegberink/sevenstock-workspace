use crate::Result;
use axum::{Json, extract::Query};
use axum_extra::headers::authorization::Bearer;
use lib_auth::token::{
    RefreshToken, Token, TokenType, config::VerifyingConfig, jwks::PublicJwkSet,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenRequest {
    grant_type: GrantType,
    refresh_token: SecretString,
    client_id: String,
    client_secret: SecretString,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    id_token: String,
}

#[derive(Deserialize)]
enum GrantType {
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
}

pub async fn api_token(Query(req): Query<TokenRequest>) -> Result<Json<TokenResponse>> {
    let refresh_token_str = req.refresh_token.expose_secret();

    let token = RefreshToken::decode(refresh_token).await?;

    let res = TokenResponse {
        access_token: "meow".into(),
        token_type: "Bearer".into(),
        expires_in: 64,
        refresh_token: "meow".into(),
        id_token: "meow".into(),
    };
    Ok(Json(res))
}
