use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_login;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/", get(handlers_login::meow))
        .with_state(mm)
}
