use std::marker::PhantomData;
pub mod config;
pub mod jwks;

use jsonwebtoken::{decode, decode_header, encode, get_current_timestamp};
mod error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::token::config::{Identifier, private_config, public_config};

pub use self::error::{Error, Result};

#[derive(Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
#[serde(transparent)]
pub struct KeyId(Uuid);

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
    TwoFactor,
}

impl TokenType {
    fn exp(&self) -> u64 {
        match self {
            TokenType::Access => 900,
            TokenType::Refresh => 604_800,
            TokenType::TwoFactor => 300,
        }
    }
}

impl TokenType {
    pub async fn verify(&self, token: &str) -> Result<Claims<Sub>> {
        let config = public_config();

        let header = decode_header(token).map_err(|_| Error::InvalidToken)?;

        let id: Identifier = (header.kid.unwrap().try_into().unwrap(), self.clone());

        let decoding_key = config.get(&id).await.unwrap();

        let token_data = decode::<Claims<Sub>>(token, &decoding_key, config.validation())
            .map_err(|_| Error::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl KeyId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Into<String> for KeyId {
    fn into(self) -> String {
        self.0.into()
    }
}

impl TryFrom<String> for KeyId {
    type Error = Error;
    fn try_from(id: String) -> Result<Self> {
        let uuid = Uuid::parse_str(&id).unwrap();
        Ok(KeyId(uuid))
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default)]
pub struct Claims<U> {
    sub: U,
    ident: Option<String>,
    avatar: Option<String>,
    exp: u64,
    iat: u64,
}

impl Claims<Sub> {
    pub fn sub(self) -> Uuid {
        self.sub.0
    }
}

pub struct TokenBuilder<U> {
    token_type: TokenType,
    claims: Claims<U>,
    _state: PhantomData<U>,
}

#[derive(Default, Clone)]
pub struct NoSub;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Sub(Uuid);

impl TokenBuilder<NoSub> {
    pub fn access() -> Self {
        Self {
            token_type: TokenType::Access,
            claims: Claims::default(),
            _state: PhantomData,
        }
    }
    pub fn refresh() -> Self {
        Self {
            token_type: TokenType::Refresh,
            claims: Claims::default(),
            _state: PhantomData,
        }
    }

    pub fn sub(self, sub: Uuid) -> TokenBuilder<Sub> {
        TokenBuilder {
            token_type: self.token_type,
            claims: Claims {
                sub: Sub(sub),
                ident: self.claims.ident,
                avatar: self.claims.avatar,
                exp: self.claims.exp,
                iat: self.claims.iat,
            },
            _state: PhantomData,
        }
    }
}

impl TokenBuilder<Sub> {
    pub fn ident<S: Into<String>>(mut self, ident: S) -> Self {
        self.claims.ident = Some(ident.into());
        self
    }

    pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
        self.claims.avatar = Some(avatar.into());
        self
    }
    pub async fn build(self) -> Result<String> {
        let config = private_config();
        let current_timestamp = get_current_timestamp();

        let claims = Claims {
            sub: self.claims.sub,
            ident: self.claims.ident,
            avatar: self.claims.avatar,
            exp: current_timestamp + self.token_type.exp(),
            iat: current_timestamp,
        };

        let fix_this_name = config.get(self.token_type).await.unwrap();

        let header = fix_this_name.header();

        let encoding_key = fix_this_name.encoding_key();

        let token = encode(header, &claims, encoding_key)
            .map_err(|_| Error::TokenEncodeFail("Failed to encode token"))?;

        Ok(token)
    }
}
