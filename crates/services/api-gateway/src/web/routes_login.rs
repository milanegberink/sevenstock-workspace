use axum::{Router, routing::post};
use lib_web::{
    handlers::{handlers_exchange, handlers_login, handlers_signup},
    services::Services,
};

pub fn routes(services: Services) -> Router {
    Router::new()
        .route(
            "/exchange-refresh",
            post(handlers_exchange::exchange_refresh),
        )
        .route("/login", post(handlers_login::api_login_handler))
        .route("/signup", post(handlers_signup::api_signup))
        .with_state(services)
}
