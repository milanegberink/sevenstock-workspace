use crate::{error::Result, services::Services};

use axum::{Json, extract::State};
use lib_core::{ctx::Ctx, model::user::UserForCreate};
use lib_grpc::SignUpRequest;
use secrecy::ExposeSecret;
use serde_json::{Value, json};

pub async fn api_signup(
    State(services): State<Services>,
    Json(user_c): Json<UserForCreate>,
) -> Result<Json<Value>> {
    let UserForCreate {
        username,
        email,
        pwd_clear,
    } = user_c;

    let res = services
        .auth()
        .sign_up(SignUpRequest {
            username,
            email,
            pwd_clear: pwd_clear.expose_secret().into(),
        })
        .await?
        .into_inner();

    let res = json!({
        "user_id": res.id
    });

    Ok(Json(res))
}
