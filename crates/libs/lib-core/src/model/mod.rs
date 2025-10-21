pub mod aws;
pub mod base;
pub mod error;
pub mod organisation;
pub mod permission;
pub mod refresh_token;
pub mod role;
pub mod store;
pub mod user;
use redis::aio::MultiplexedConnection;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::model::store::dbx::Dbx;
use crate::{config, model::store::new_db_pool};

pub use self::error::{Error, Result};

pub type Db = Pool<Postgres>;

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
    redis: MultiplexedConnection,
    http_client: reqwest::Client,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        {
            let config = config::core_config();

            let db_pool = new_db_pool()
                .await
                .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;

            let dbx = Dbx::new(db_pool, false)?;

            let pg_version = "1";

            info!(
                "Connected to database with Postgres version: {} ",
                pg_version
            );

            let client = redis::Client::open(config.redis_url())?;

            let redis = client.get_multiplexed_tokio_connection().await?;

            let http_client = reqwest::Client::new();

            Ok(Self {
                dbx,
                redis,
                http_client,
            })
        }
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        Ok(ModelManager {
            dbx: Dbx::new(self.dbx.db().clone(), true)?,
            redis: self.redis(),
            http_client: self.http_client.clone(),
        })
    }

    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }
    pub fn redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
}
