use crate::config::auth_config;
use jsonwebtoken::{DecodingKey, EncodingKey, TokenData, decode, encode, get_current_timestamp};
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
struct Claims {
    sub: Uuid,
    ident: Option<String>,
    avatar: Option<String>,
    exp: u64,
    iat: u64,
}

pub struct TokenBuilder {
    token: Token,
    claims: Claims,
}

impl TokenBuilder {
    pub fn sub(mut self, sub: Uuid) -> Self {
        self.claims.sub = sub;
        self
    }

    pub fn ident<S: Into<String>>(mut self, ident: S) -> Self {
        self.claims.ident = Some(ident.into());
        self
    }

    pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
        self.claims.avatar = Some(avatar.into());
        self
    }
}

impl TokenBuilder {
    pub fn access() -> Self {
        Self {
            token: ACCESS_TOKEN,
            claims: Claims::default(),
        }
    }
    pub fn refresh() -> Self {
        Self {
            token: REFRESH_TOKEN,
            claims: Claims::default(),
        }
    }
    pub fn build(self) -> Result<String> {
        let config = auth_config();
        let current_timestamp = get_current_timestamp();

        if self.claims.sub.is_nil() {
            return Err(Error::TokenEncodeFail("Sub field is nil"));
        }

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
    fn verify(&self, token: &str) -> Result<TokenData<Claims>> {
        let config = auth_config();

        let claims = decode::<Claims>(token, &(self.decoding_key)(), config.jwt_ctx().validation())
            .map_err(|_| Error::InvalidToken)?;

        Ok(claims)
    }
}
