use crate::model::Result;
use crate::model::base;
use modql::field::Fields;
use secrecy::zeroize;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    generate_common_bmc_fns,
    model::{ModelManager, base::DbBmc},
};

#[derive(Clone, Fields, Serialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
}

#[derive(Fields, Deserialize)]
pub struct ApiKeyForCreate {
    pub owner_id: Uuid,
}

impl DbBmc for ApiKeyBmc {
    const TABLE: &'static str = "api_keys";

    fn has_owner_id() -> bool {
        true
    }
}

pub struct ApiKeyBmc;

generate_common_bmc_fns!(
    Bmc: ApiKeyBmc,
    Entity: ApiKey,
    ForCreate: ApiKeyForCreate,
);
