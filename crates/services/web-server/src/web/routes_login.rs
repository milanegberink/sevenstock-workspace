use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::{handlers_exchange, handlers_login};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/exchange-refresh",
            post(handlers_exchange::exchange_refresh),
        )
        .route("/login", post(handlers_login::api_login_handler))
        .with_state(mm)
}
