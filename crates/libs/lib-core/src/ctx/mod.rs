use std::collections::HashMap;

use uuid::Uuid;
mod error;
use crate::model::permission::{Action, Permissions, Resource};

pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: Uuid,
}

impl Ctx {
    pub fn new(user_id: Uuid) -> Result<Self> {
        Ok(Self { user_id })
    }
    pub fn root_ctx() -> Self {
        Ctx {
            user_id: Uuid::nil(),
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
