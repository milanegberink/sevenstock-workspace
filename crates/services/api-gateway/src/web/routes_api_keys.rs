use axum::{Router, routing::post};
use lib_web::{handlers::handlers_api_keys, services::Services};

pub fn routes(services: Services) -> Router {
    Router::new()
        .route("/new-api-key", post(handlers_api_keys::new_api_key))
        .with_state(services)
}
