use axum::{
    Json,
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::{TypedHeader, headers::Referer};
use lib_auth::{pwd::verify_password, token::TokenType};
use lib_core::{
    ctx::Ctx,
    model::{
        ModelManager,
        user::{UserBmc, UserForLogin},
    },
};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_email::Email;
use serde_json::Value;
use tracing::info;

use crate::{Error, Result};

#[derive(Deserialize)]
pub struct LoginPayload {
    email: Email,
    pwd: SecretString,
}

#[derive(Deserialize)]
pub struct X {
    token: String,
}

pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    TypedHeader(referer): TypedHeader<Referer>,
    Json(req): Json<LoginPayload>,
) -> Result<()> {
    let LoginPayload {
        email,
        pwd: pwd_clear,
    } = req;

    info!("{}", referer);

    let root_ctx = Ctx::root_ctx();

    let UserForLogin {
        username: _username,
        id,
        pwd: pwd_hash,
    } = UserBmc::first_by_email(&root_ctx, &mm, &email).await?;

    let Some(pwd) = pwd_hash else {
        return Err(Error::LoginFailUserHasNoPwd { user_id: id });
    };

    verify_password(&pwd, &pwd_clear.expose_secret())?;

    Ok(())
}
