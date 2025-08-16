use axum::{Router, routing::get};
use lib_web::handlers::handlers_login;

pub fn routes() -> Router {
    Router::new().route("/", get(handlers_login::meow))
}
