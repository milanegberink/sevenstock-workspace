use serde::Serialize;
use uuid::Uuid;
mod error;
pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    sub: Uuid,
}

impl Ctx {
    pub fn new(sub: Uuid) -> Result<Self> {
        Ok(Self { sub })
    }
}

impl Ctx {
    pub fn sub(&self) -> &Uuid {
        &self.sub
    }
}
