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

pub trait OrganisationBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl OrganisationBy for Organisation {}

pub struct OrganisationBmc;

#[derive(Debug, Serialize, Deserialize, Fields, FromRow)]
pub struct Organisation {
    id: Uuid,
    name: String,
}

#[derive(Deserialize, Fields)]
pub struct OrganisationForCreate {
    name: String,
}

impl DbBmc for OrganisationBmc {
    const TABLE: &'static str = "organisation";
}

generate_common_bmc_fns!(
    Bmc: OrganisationBmc,
    Entity: Organisation,
    ForCreate: OrganisationForCreate,
);
