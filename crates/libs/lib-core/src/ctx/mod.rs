use std::collections::HashMap;

use uuid::Uuid;
mod error;
use crate::model::permission::{Action, Permissions, Resource};

pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: Uuid,
    org_id: Option<Uuid>,
}

impl Ctx {
    pub fn new(user_id: Uuid, org_id: Option<Uuid>) -> Result<Self> {
        Ok(Self { user_id, org_id })
    }
    pub fn root_ctx() -> Self {
        Ctx {
            user_id: Uuid::nil(),
            org_id: None,
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
