use axum::{Router, routing::post};
use lib_core::model::ModelManager;
use lib_web::handlers::{handlers_exchange, handlers_login, handlers_signup};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/exchange-refresh",
            post(handlers_exchange::exchange_refresh),
        )
        .route("/login", post(handlers_login::api_login_handler))
        .route("/signup", post(handlers_signup::api_signup))
        .route("/get-user", post(handlers_signup::get_user))
        .with_state(mm)
}
