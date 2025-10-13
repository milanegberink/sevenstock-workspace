use std::collections::HashMap;

use uuid::Uuid;
mod error;
use crate::model::permission::{Action, Permissions, Resource};

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct Ctx {
    user_id: Uuid,
    org_id: Uuid,
    permissions: Permissions,
}

impl Ctx {
    pub fn new(user_id: Uuid, org_id: Uuid, permissions: Permissions) -> Result<Self> {
        Ok(Self {
            user_id,
            org_id,
            permissions,
        })
    }
    pub fn root_ctx() -> Self {
        Ctx {
            user_id: Uuid::nil(),
            permissions: HashMap::new(),
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }
    pub fn org_id(&self) -> Uuid {
        self.org_id
    }
}
