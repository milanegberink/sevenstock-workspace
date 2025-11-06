use axum::{Json, extract::State};
use lib_core::model::ModelManager;

use crate::Result;

pub struct OpenIDConfig {
    // Required fields
    issuer: String,
    authorization_endpoint: String,
    jwks_uri: String,
    response_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,

    // Optional fields
    token_endpoint: String,
    userinfo_endpoint: String,
    scopes_supported: Vec<String>,
    token_endpoint_auth_methods_supported: Vec<String>,
    claims_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    response_modes_supported: Vec<String>,
    code_challenge_methods_supported: Vec<String>,
}

// pub async fn api_public_jwks() -> Result<Json<JwkSet>> {
//     let config = VerifyingConfig::get()?;
//     let jwks = config.pub_jwk_set();

//     Ok(Json(jwks.clone()))
// }

// pub fn api_openid_config(State(mm): State<ModelManager>) -> Result<Json<PublicJwkSet>> {
//     let config = VerifyingConfig::get()?;
//     let jwks = config.pub_jwk_set();

//     Ok(Json(jwks.clone()))
// }
