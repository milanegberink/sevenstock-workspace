pub mod aws;
pub mod error;
pub mod user;

use std::sync::Arc;

use lib_grpc::{AuthClient, Channel};
use redis::aio::MultiplexedConnection;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

use crate::config;

pub use self::error::{Error, Result};

pub type Db = Pool<Postgres>;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
    redis: MultiplexedConnection,
    http_client: reqwest::Client,
    services: Arc<Services>,
}

pub struct Services {
    auth: AuthClient<Channel>,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        // #[cfg(debug_assertions)]
        {
            let config = config::core_config();

            let db = PgPoolOptions::new().connect(config.db_url()).await?;

            let pg_version = sqlx::query!("SELECT version()")
                .fetch_one(&db)
                .await?
                .version
                .unwrap();

            info!(
                "Connected to database with Postgres version: {} ",
                pg_version
            );

            let client = redis::Client::open(config.redis_url())?;

            let redis = client.get_multiplexed_tokio_connection().await?;

            let http_client = reqwest::Client::new();

            let auth_client = AuthClient::connect(config.auth_grpc()).await.unwrap();

            let services = Services { auth: auth_client };

            Ok(Self {
                db,
                redis,
                http_client,
                services: Arc::new(services),
            })
        }

        // #[cfg(not(debug_assertions))]
        // {
        //     todo!()
        // }
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
    pub fn redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
    pub fn auth(&self) -> AuthClient<Channel> {
        self.services.auth.clone()
    }
}
