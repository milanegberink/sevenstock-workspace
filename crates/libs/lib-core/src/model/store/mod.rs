pub(in crate::model) mod dbx;

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    PgPoolOptions::new().connect(&core_config().db_url()).await
}
