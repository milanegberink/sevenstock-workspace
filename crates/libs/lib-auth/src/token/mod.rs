use std::sync::Arc;

use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, decode_header, encode,
    get_current_timestamp,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use strum_macros::Display;
use uuid::Uuid;
mod config;
mod error;

pub use self::error::{Error, Result};

use crate::token::config::{SigningConfig, VerifyingConfig};

pub const ACCESS_TOKEN_TTL: u64 = 900;
pub const REFRESH_TOKEN_TTL: u64 = 9000;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Display, Deserialize, Copy)]
pub enum TokenType {
    Access,
    Refresh,
}

pub trait Token: Serialize + DeserializeOwned + Sized {
    const TOKEN_TYPE: TokenType;
    async fn encode(&self) -> Result<String> {
        let config = SigningConfig::get()?;

        let jwt_header = config.get_encoding_key(Self::TOKEN_TYPE).await?;
        let token = encode(jwt_header.header(), &self, jwt_header.encoding_key())?;

        Ok(token)
    }
    async fn decode(token: &str) -> Result<Self> {
        let config = VerifyingConfig::get()?;

        let header = decode_header(token)?;

        let kid_string = header.kid.ok_or(Error::InvalidToken)?;

        let kid = Uuid::parse_str(&kid_string).map_err(|_| Error::InvalidToken)?;

        let decoding_key = config.get_decoding_key(&(kid, Self::TOKEN_TYPE)).await?;

        let token_date = decode::<Self>(token, &decoding_key, config.validation())?;

        Ok(token_date.claims)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Copy)]
pub struct AccessToken {
    sub: Uuid,
    #[serde(flatten)]
    std_claims: StandardClaims,
}

impl Token for AccessToken {
    const TOKEN_TYPE: TokenType = TokenType::Access;
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Copy)]
struct StandardClaims {
    iat: u64,
    exp: u64,
}

impl AccessToken {
    pub fn new(sub: Uuid) -> Self {
        AccessToken {
            sub,
            std_claims: StandardClaims::new(ACCESS_TOKEN_TTL),
        }
    }
}

impl StandardClaims {
    fn new(ttl: u64) -> Self {
        let current_timestamp = get_current_timestamp();
        Self {
            iat: current_timestamp,
            exp: current_timestamp + ttl,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Copy, Deserialize)]
pub struct RefreshToken {
    sub: Uuid,
    #[serde(flatten)]
    std_claims: StandardClaims,
}

impl RefreshToken {
    pub fn new(sub: Uuid) -> Self {
        RefreshToken {
            sub,
            std_claims: StandardClaims::new(ACCESS_TOKEN_TTL),
        }
    }
}

impl Token for RefreshToken {
    const TOKEN_TYPE: TokenType = TokenType::Refresh;
}
