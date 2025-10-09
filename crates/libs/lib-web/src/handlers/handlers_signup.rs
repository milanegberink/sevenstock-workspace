use crate::{error::Result, services::Services};

use axum::{Json, extract::State};
use lib_core::ctx::Ctx;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn api_signup(
    State(mm): State<Services>,
    Json(user_c): Json<UserForCreate>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();

    let res = json!({
        "user_id": id
    });

    Ok(Json(res))
}

#[derive(serde::Deserialize)]
pub struct GetUser {
    id: Uuid,
}

pub async fn get_user(
    State(mm): State<Services>,
    Json(user_c): Json<GetUser>,
) -> Result<Json<User>> {
    let ctx = Ctx::root_ctx();

    Ok(Json(user))
}
