use lib_proc_macros::DbEntity;
use redis::AsyncCommands;
use redis::RedisError;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::{ctx::Ctx, model::ModelManager};

#[derive(DbEntity, Debug, Serialize, Deserialize)]
#[table_name("users")]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
