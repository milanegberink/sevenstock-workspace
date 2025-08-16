mod config;

use crate::web::routes_login;

pub use self::error::{Error, Result};

mod web;

use axum::{Router, middleware};
use lib_core::model::ModelManager;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
mod error;
use lib_web::middleware::mw_auth::mw_ctx_require;
use tower_http::trace::TraceLayer;
use tracing_appender::rolling;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = rolling::daily(".logs", "app.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = fmt::layer().with_target(false).with_writer(std::io::stdout);

    let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
        .with(console_layer)
        .with(file_layer)
        .init();

    let mm = ModelManager::new().await?;

    let app = Router::new()
        .with_state(mm)
        .merge(routes_login::routes())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(mw_ctx_require));

    let addr = format!("0.0.0.0:{}", PORT);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!(
        "Server started on address: {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
