use crate::{
    ctx::Ctx,
    model::{Result, user::UserBmc},
};
use std::collections::{HashMap, HashSet};

use modql::field::Fields;
use sea_query::{Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{Type, prelude::FromRow};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::model::{ModelManager, base::DbBmc};

pub type Permissions = HashMap<Resource, HashSet<Action>>;

#[derive(EnumString, Clone, Display, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Resource {
    Products,
    Orders,
}

impl From<Resource> for sea_query::Value {
    fn from(val: Resource) -> Self {
        val.to_string().into()
    }
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Clone, Display, sqlx::Type, Deserialize, Serialize, Debug)]
#[sqlx(type_name = "action_permission")]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    READ,
    WRITE,
    DELETE,
    CREATE,
    #[strum(serialize = "*")]
    #[sqlx(rename = "all")]
    ALL,
}

impl From<Action> for sea_query::Value {
    fn from(val: Action) -> Self {
        val.to_string().into()
    }
}

#[derive(Iden)]
pub enum PermissionIden {
    Resource,
    Action,
}

#[derive(Debug, Serialize, Deserialize, Fields, FromRow)]
pub struct Permission {
    resource: Resource,
    action: Action,
}

pub struct PermissionBmc;

impl DbBmc for PermissionBmc {
    const TABLE: &'static str = "permission";
}
