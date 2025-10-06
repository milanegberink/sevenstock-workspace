use crate::error::Result;

use axum::{Json, extract::State};
use lib_auth::api_key::{self, generate_api_key, hash_api_key};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::debug;

use crate::middleware::mw_auth::CtxW;

pub async fn gen_api_key(State(mm): State<ModelManager>) -> Result<Json<Value>> {
    // let ctx = ctx.0;

    let api_key = generate_api_key();

    let hash = hash_api_key(&api_key);
    debug!("{}", hash);

    let res = json!({
        "api_key": api_key
    });

    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct Test {
    hash: String,
    raw: String,
}

pub async fn verify_api_key(Json(payload): Json<Test>) -> Result<()> {
    api_key::verify_api_key(&payload.raw, &payload.hash).unwrap();

    Ok(())
}
