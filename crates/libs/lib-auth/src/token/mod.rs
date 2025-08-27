use std::marker::PhantomData;
mod config;
pub mod secrets;

use crate::config::auth_config;
use jsonwebtoken::{
    DecodingKey, EncodingKey, decode, decode_header, encode, get_current_timestamp,
};
mod error;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub use self::error::{Error, Result};

pub const ACCESS_TOKEN: Token = Token {
    expiration: 900,
    encoding_key: || auth_config().keys().access().encoding(),
    decoding_key: || auth_config().keys().access().decoding(),
};

pub const REFRESH_TOKEN: Token = Token {
    expiration: 604_800,
    encoding_key: || auth_config().keys().refresh().encoding(),
    decoding_key: || auth_config().keys().refresh().decoding(),
};

pub const TEMP_2FA_TOKEN: Token = Token {
    expiration: 900,
    encoding_key: || auth_config().keys().auth_2fa().encoding(),
    decoding_key: || auth_config().keys().auth_2fa().decoding(),
};

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
    token: Token,
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
            token: ACCESS_TOKEN,
            claims: Claims::default(),
            _state: PhantomData,
        }
    }
    pub fn refresh() -> Self {
        Self {
            token: REFRESH_TOKEN,
            claims: Claims::default(),
            _state: PhantomData,
        }
    }

    pub fn sub(self, sub: Uuid) -> TokenBuilder<Sub> {
        TokenBuilder {
            token: self.token,
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
    pub fn build(self) -> Result<String> {
        let config = auth_config();
        let current_timestamp = get_current_timestamp();

        let claims = Claims {
            sub: self.claims.sub,
            ident: self.claims.ident,
            avatar: self.claims.avatar,
            exp: current_timestamp + self.token.expiration,
            iat: current_timestamp,
        };

        let token = encode(
            config.jwt_ctx().header(),
            &claims,
            (self.token.encoding_key)(),
        )
        .map_err(|_| Error::TokenEncodeFail("Failed to encode token"))?;

        Ok(token)
    }
}

pub struct Token {
    expiration: u64,
    encoding_key: fn() -> &'static EncodingKey,
    decoding_key: fn() -> &'static DecodingKey,
}

impl Token {
    pub fn verify(&self, token: &str) -> Result<Claims<Sub>> {
        let config = auth_config();

        let token_data =
            decode::<Claims<Sub>>(token, (self.decoding_key)(), config.jwt_ctx().validation())
                .map_err(|_| Error::InvalidToken)?;

        Ok(token_data.claims)
    }
}
