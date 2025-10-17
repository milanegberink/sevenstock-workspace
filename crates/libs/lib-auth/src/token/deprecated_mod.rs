pub mod config;
pub mod jwks;

use jsonwebtoken::{decode, decode_header, encode, get_current_timestamp};
mod error;
use crate::oauth2::OAuthQuery;
use crate::token::config::{Identifier, VerifyingConfig, signing_config};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;
use uuid::Uuid;

pub use self::error::{Error, Result};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug, Display)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
    OAuth2,
}

impl TokenType {
    fn exp(&self) -> u64 {
        match self {
            Self::Access => 900,
            Self::Refresh => 604_800,
            Self::OAuth2 => 600,
        }
    }
}

impl TokenType {
    pub async fn verify(&self, token: &str) -> Result<Claims> {
        let config = VerifyingConfig::get()?;

        let header = decode_header(token).map_err(|_| Error::InvalidToken)?;

        let id: Identifier = (header.kid.unwrap().try_into().unwrap(), self.clone());

        let decoding_key = config.get_decoding_key(&id).await?;

        let token_data = decode::<Claims>(token, &decoding_key, config.validation())
            .map_err(|_| Error::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Claims {
    sub: Option<Uuid>,
    ident: Option<String>,
    avatar: Option<String>,
    exp: u64,
    iat: u64,
    iss: Option<String>,
    org: Option<String>,
    scope: Option<String>,
    oauth2_params: Option<OAuthQuery>,
}

impl Claims {
    pub fn sub(&self) -> &Option<Uuid> {
        &self.sub
    }
    pub fn scope(&self) -> &Option<String> {
        &self.scope
    }
}

#[cfg(feature = "private")]
pub struct Token {
    token_type: TokenType,
    claims: Claims,
}

#[derive(Default, Clone)]
pub struct NoSub;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Sub(Uuid);

#[cfg(feature = "private")]
impl Token {
    pub fn access() -> Self {
        Self {
            token_type: TokenType::Access,
            claims: Claims::default(),
        }
    }
    pub fn refresh() -> Self {
        Self {
            token_type: TokenType::Refresh,
            claims: Claims::default(),
        }
    }

    pub fn oauth2() -> Self {
        Self {
            token_type: TokenType::OAuth2,
            claims: Claims::default(),
        }
    }

    pub fn sub(self, sub: &Uuid) -> Token {
        Token {
            token_type: self.token_type,
            claims: Claims {
                sub: Some(*sub),
                ident: self.claims.ident,
                avatar: self.claims.avatar,
                exp: self.claims.exp,
                iat: self.claims.iat,
                org: self.claims.org,
                iss: self.claims.iss,
                scope: self.claims.scope,
                oauth2_params: self.claims.oauth2_params,
            },
        }
    }
}

#[cfg(feature = "private")]
impl Token {
    pub fn ident<S: Into<String>>(mut self, ident: S) -> Self {
        self.claims.ident = Some(ident.into());
        self
    }

    pub fn scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.claims.scope = Some(scope.into());

        self
    }

    pub fn iss<S: Into<String>>(mut self, iss: S) -> Self {
        self.claims.iss = Some(iss.into());
        self
    }

    pub fn org<S: Into<String>>(mut self, org: S) -> Self {
        self.claims.org = Some(org.into());
        self
    }

    pub fn oauth2_params(mut self, params: OAuthQuery) -> Self {
        self.claims.oauth2_params = Some(params);
        self
    }

    pub fn avatar<S: Into<String>>(mut self, avatar: S) -> Self {
        self.claims.avatar = Some(avatar.into());
        self
    }

    pub async fn build_async(self) -> Result<String> {
        let config = signing_config()?;
        let current_timestamp = get_current_timestamp();

        let claims = Claims {
            sub: self.claims.sub,
            ident: self.claims.ident,
            avatar: self.claims.avatar,
            exp: current_timestamp + self.token_type.exp(),
            org: self.claims.org,
            scope: self.claims.scope,
            iss: self.claims.iss,
            iat: self.claims.iat,
            oauth2_params: self.claims.oauth2_params,
        };

        let jwt_header = config
            .get(self.token_type)
            .await
            .ok_or(Error::NoHeaderFound {
                token_type: self.token_type,
            })?;

        let header = jwt_header.header();

        let encoding_key = jwt_header.encoding_key();

        let token = encode(header, &claims, encoding_key)?;

        Ok(token)
    }
}
