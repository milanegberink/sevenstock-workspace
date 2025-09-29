mod config;

use std::env;

use crate::web::routes_login;

pub use self::error::{Error, Result};

mod web;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    },
    middleware,
};
use lib_core::model::ModelManager;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
mod error;
use lib_web::middleware::{
    mw_auth::{mw_ctx_require, mw_ctx_resolver},
    mw_rate_limiter::mw_rate_limiter,
};
use tower_http::trace::TraceLayer;
use tracing_appender::rolling;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let mm = ModelManager::new().await?;

    let temp_routes = Router::new().merge(routes_login::routes(mm.clone()));

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_origin([HeaderValue::from_static("http://localhost:5173")])
        .allow_headers([CONTENT_TYPE, COOKIE, AUTHORIZATION]);

    let app = Router::new()
        .nest("/auth", temp_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors);
    // .layer(middleware::from_fn_with_state(mm.clone(), mw_rate_limiter))
    // .layer(middleware::from_fn(mw_ctx_require))
    // .layer(middleware::from_fn_with_state(mm, mw_ctx_resolver));

    let port: u16 = env::var("BACKEND_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5000);

    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn init_logging() {
    let file_appender = rolling::daily(".logs", "app.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer().with_target(false).with_writer(std::io::stdout);

    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(console_layer)
        .with(file_layer)
        .init();
}
