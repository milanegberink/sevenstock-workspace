pub mod aws;
pub mod error;
pub mod user;

use lib_grpc::AuthClient;
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
    services: Services,
}

pub struct Services {
    auth: lib_grpc::AuthClient,
}

impl Services {
    pub async fn new() -> Self {
        Self {
            auth: AuthClient::connect("http://[::1]:50051").await.unwrap(),
        }
    }
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
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

            Ok(Self {
                db,
                redis,
                http_client,
            })
        }

        #[cfg(not(debug_assertions))]
        {
            todo!()
        }
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
    pub fn aws(&self) -> &Aws {
        &self.aws
    }
}
