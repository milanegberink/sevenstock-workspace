pub mod error;
pub mod repositories;

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
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
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

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
    pub fn redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
}
