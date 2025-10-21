use crate::{Result, handlers::handlers_oauth2::token};
use axum::{Json, extract::Query};
use axum_extra::headers::authorization::Bearer;
use lib_auth::{
    secret::{generate_secret_key, hash_secret_key},
    token::{
        AccessToken, Jwt, RefreshToken, TokenType, config::VerifyingConfig, jwks::PublicJwkSet,
    },
};
use lib_core::{
    ctx::Ctx,
    model::refresh_token::{RefreshToken, RefreshTokenBmc, RefreshTokenForCreate},
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    refresh_token: Option<String>,
    id_token: Option<String>,
}

#[derive(Deserialize)]
enum GrantType {
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
}

pub async fn api_token(Query(req): Query<TokenRequest>) -> Result<Json<TokenResponse>> {
    match req.grant_type {
        GrantType::RefreshToken => {
            let ctx = Ctx::root_ctx();

            let token_hash = hash_secret_key(&req.refresh_token.expose_secret())?;

            let refresh_token: RefreshToken =
                RefreshTokenBmc::get_by_hash(&ctx, &mm, &token_hash).await?;

            let access_token = AccessToken::new(refresh_token.user_id).encode().await?;
        }
        GrantType::AuthorizationCode => {
            let ctx = Ctx::root_ctx();

            let token_hash = hash_secret_key(&req.refresh_token.expose_secret())?;

            let old_token: RefreshToken =
                RefreshTokenBmc::get_by_hash(&ctx, &mm, token_hash).await?;

            let ctx = Ctx::new(old_token.user_id, None)?;

            let refresh_token = RefreshTokenBmc::generate_new(&ctx, &mm).await?;

            let ctx = Ctx::new(user_id, org_id)?;

            let raw_token = RefreshTokenBmc::generate_new(&ctx, &mm).await?;

            let access_token = AccessToken::new(ctx.user_id());

            let res = TokenResponse {
                access_token: access_token.encode()?,
                token_type: "Bearer".into(),
                refresh_token: refresh_token.expose_secret(),
                expires_in: access_token.expires_at(),
                id_token: None,
            };

            Ok(Json(res))
        }
    }

    let res = TokenResponse {
        access_token: "meow".into(),
        token_type: "Bearer".into(),
        expires_in: 64,
        refresh_token: "meow".into(),
        id_token: "meow".into(),
    };
    Ok(Json(res))
}
