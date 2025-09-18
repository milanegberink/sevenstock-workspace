mod config;

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
use lib_auth::token::{
    TokenType,
    config::{init_signing_config, init_verifying_config},
    jwks::{PrivateJwk, PrivateJwkSet},
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

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let mm = ModelManager::new().await?;

    let temp_routes = Router::new().merge(routes_login::routes(mm.clone()));

    let private_jwk = PrivateJwk::new(TokenType::Access);

    let private_jwk_2 = PrivateJwk::new(TokenType::Refresh);

    let mut set = PrivateJwkSet { keys: Vec::new() };

    set.keys.push(private_jwk);

    set.keys.push(private_jwk_2);

    init_signing_config(set).unwrap();

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

    let addr = format!("0.0.0.0:{}", PORT);

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
