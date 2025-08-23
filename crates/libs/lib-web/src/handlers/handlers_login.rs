use crate::error::{Error, Result};

use axum::extract::State;
use lib_auth::token::{Token, TokenBuilder, secrets::KeyPair};
use lib_core::model::ModelManager;
use tracing::info;
use uuid::Uuid;

pub async fn meow(State(mm): State<ModelManager>) -> Result<&'static str> {
    let aws = mm.aws();

    let secret = aws
        .secrets()
        .get_secret_value()
        .secret_id("0198c832-5c00-7ad2-8046-f8e9f0206efc")
        .send()
        .await?;

    info!("{:?}", secret.secret_string());

    Ok("string")
}
