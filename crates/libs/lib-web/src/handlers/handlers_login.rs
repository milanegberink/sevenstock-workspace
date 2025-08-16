use crate::error::{Error, Result};

use lib_auth::token::{Token, TokenBuilder};
use uuid::Uuid;

pub async fn meow() -> Result<String> {
    let token = TokenBuilder::access().sub(Uuid::now_v7()).build()?;

    Ok(token)
}
