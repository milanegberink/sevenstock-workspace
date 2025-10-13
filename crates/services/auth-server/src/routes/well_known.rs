pub fn routes() -> Router {
    Router::new().route("/openid-configuration", method_router)
}
