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
pub mod jwks;

pub use self::error::{Error, Result};

use crate::token::config::{SigningConfig, VerifyingConfig};

pub const ACCESS_TOKEN_TTL: u64 = 900;
pub const REFRESH_TOKEN_TTL: u64 = 9000;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Display, Deserialize, Copy)]
pub enum TokenType {
    Access,
    IDToken,
}

pub trait Jwt: Serialize + DeserializeOwned + Sized {
    fn encode(&self) -> Result<String> {
        let config = SigningConfig::get()?;

        let encoding_key = config.encoding_key();

        let header = config.header();

        let token = encode(header, &self, encoding_key)?;

        Ok(token)
    }
    fn decode(token: &str) -> Result<Self> {
        let config = VerifyingConfig::get()?;

        let header = decode_header(token)?;

        let kid_string = header.kid.ok_or(Error::InvalidToken)?;

        let kid = Uuid::parse_str(&kid_string).map_err(|_| Error::InvalidToken)?;

        let decoding_key = config.get_decoding_key(kid)?;

        let token_date = decode::<Self>(token, &decoding_key, config.validation())?;

        Ok(token_date.claims)
    }
}

impl Jwt for AccessToken {}

impl Jwt for IDToken {}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Copy)]
pub struct AccessToken {
    pub sub: Uuid,
    #[serde(flatten)]
    std_claims: StandardClaims,
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
    pub fn expires_at(&self) -> u64 {
        self.std_claims.exp
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
pub struct IDToken {
    pub sub: Uuid,
    #[serde(flatten)]
    std_claims: StandardClaims,
}

impl IDToken {
    pub fn new(sub: Uuid) -> Self {
        IDToken {
            sub,
            std_claims: StandardClaims::new(REFRESH_TOKEN_TTL),
        }
    }
}
