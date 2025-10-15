use axum::{
    Router,
    routing::{get, post},
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_oauth2;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/authorize",
            get(handlers_oauth2::authorize::oauth2_authorize_handler),
        )
        .route("/token", post(handlers_oauth2::token::api_token))
        .with_state(mm)
}
