use crate::{error::Result, services::Services};

use axum::{Json, extract::State};
use lib_auth::api_key::{self, generate_api_key, hash_api_key};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::debug;

use crate::middleware::mw_auth::CtxW;

pub async fn new_api_key(State(services): State<Services>) -> Result<Json<Value>> {
    // let ctx = ctx.0;

    let api_key = generate_api_key();

    let hash = hash_api_key(api_key);
    debug!("{}", hash);

    let res = json!({
        "api_key": "x"
    });

    Ok(Json(res))
}

pub async fn delete_api_key(State(services): State<Services>) -> Result<Json<Value>> {
    todo!();
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
