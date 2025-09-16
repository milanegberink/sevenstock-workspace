use crate::error::{Error, Result};

use axum::extract::State;
use lib_auth::token::{Token, TokenBuilder, secrets::KeyPair};
use lib_core::model::ModelManager;
use secrecy::SecretString;
use tracing::info;
use uuid::Uuid;

pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<LoginPayload>,
) -> Result<&'static str> {
    let LoginPayload {
        email,
        pwd: pwd_clear,
    } = payload;

    info!("{:?}", secret.secret_string());

    Ok("string")
}

struct LoginPayload {
    email: String,
    pwd: SecretString,
}
