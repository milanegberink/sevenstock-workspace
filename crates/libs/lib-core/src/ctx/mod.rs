use serde::Serialize;
use uuid::Uuid;
mod error;
pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
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
