mod config;

use std::{env, path::PathBuf};

use crate::web::{routes_login, routes_oauth2, routes_well_known};

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
    config::{SigningConfig, VerifyingConfig},
    jwks::{JwkSet, PrivateJwk, PrivateJwkSet, PublicJwk},
};
use lib_core::model::ModelManager;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
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

    let private_jwk = PrivateJwk::new(TokenType::OAuth2);

    let public_jwk = private_jwk.public.clone();

    let public_set = JwkSet {
        keys: vec![public_jwk],
    };

    let private_set = PrivateJwkSet {
        keys: vec![private_jwk],
    };

    VerifyingConfig::init(public_set).unwrap();

    SigningConfig::init(private_set).unwrap();

    let mm = ModelManager::new().await?;

    let manifest_sir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //
    let workspace_root = manifest_sir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let static_files_path = workspace_root.join("web-folder/apps/oauth-login/build");

    println!("Serving static files from: {}", static_files_path.display());

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
        .nest("/oauth2", routes_oauth2::routes(mm.clone()))
        .nest_service(
            "/login",
            ServeDir::new(static_files_path).append_index_html_on_directories(true),
        )
        .nest("/api", routes_login::routes(mm))
        .merge(routes_well_known::routes())
        .layer(TraceLayer::new_for_http())
        // .layer(middleware::from_fn_with_state(mm.clone(), mw_rate_limiter))
        // .layer(middleware::from_fn(mw_ctx_require))
        // .layer(middleware::from_fn_with_state(mm, mw_ctx_resolver))
        .layer(cors);

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
