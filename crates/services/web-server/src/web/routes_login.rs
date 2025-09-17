use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_login;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/login", post(handlers_login::api_login_handler))
        .with_state(mm)
}
