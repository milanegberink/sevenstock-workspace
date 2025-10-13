use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base;
use crate::{generate_common_bmc_fns, model::base::DbBmc};
use modql::field::{Fields, HasSeaFields};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub trait RoleBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RoleBy for Role {}

pub struct RoleBmc;

#[derive(Debug, Serialize, Deserialize, Fields, FromRow)]
pub struct Role {
    id: Uuid,
    name: String,
}

#[derive(Deserialize, Fields)]
pub struct RoleForCreate {
    name: String,
    organisation_id: Uuid,
}

impl DbBmc for RoleBmc {
    const TABLE: &'static str = "role";
}

generate_common_bmc_fns!(
    Bmc: RoleBmc,
    Entity: Role,
    ForCreate: RoleForCreate,
);
