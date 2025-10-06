use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_api_keys;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/new-api-key", post(handlers_api_keys::gen_api_key))
        .route("/verify-api-key", post(handlers_api_keys::verify_api_key))
        .with_state(mm)
}
