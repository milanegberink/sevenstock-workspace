use crate::error::Result;

use axum::{Json, extract::State};
use lib_auth::api_key::{generate_api_key, hash_api_key};
use lib_core::model::ModelManager;
use serde_json::{Value, json};

use crate::middleware::mw_auth::CtxW;

pub async fn gen_api_key(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Json<Value>> {
    let ctx = ctx.0;

    let api_key = generate_api_key();

    let hash = hash_api_key(&api_key);

    let res = json!({
        "api_key": api_key
    });

    Ok(Json(res))
}
