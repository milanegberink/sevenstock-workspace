use crate::token::{Error, Result, TokenType};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use tokio::sync::RwLock;

use crate::token::{
    KeyId,
    jwks::{PrivateJwkSet, PublicJwkSet},
};

static PUBLIC_INSTANCE: OnceLock<PublicTokenConfig> = OnceLock::new();
static PRIVATE_INSTANCE: OnceLock<PrivateTokenConfig> = OnceLock::new();

pub fn init_public_config(set: PublicJwkSet) -> Result<()> {
    let public_config = set.try_into().map_err(|_| Error::InvalidJwkSet)?;
    PUBLIC_INSTANCE.set(public_config);

    Ok(())
}

pub fn init_private_config(set: PrivateJwkSet) -> Result<()> {
    let private_config = set.try_into().map_err(|_| Error::InvalidJwkSet)?;
    PRIVATE_INSTANCE.set(private_config);

    Ok(())
}

pub fn public_config() -> &'static PublicTokenConfig {
    PUBLIC_INSTANCE.get().expect("No public config initialised")
}

pub fn private_config() -> &'static PrivateTokenConfig {
    PRIVATE_INSTANCE
        .get()
        .expect("No private config initialised")
}

pub struct PublicTokenConfig {
    validation: Validation,
    keys: RwLock<HashMap<Identifier, Arc<DecodingKey>>>,
}

impl PublicTokenConfig {
    pub async fn get(&self, id: &Identifier) -> Option<Arc<DecodingKey>> {
        let map = self.keys.read().await;
        map.get(id).cloned()
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
}

impl PrivateTokenConfig {
    pub async fn get(&self, token_type: TokenType) -> Option<Arc<JwtHeader>> {
        let map = self.private.read().await;
        map.get(&token_type).cloned()
    }
}

pub type Identifier = (KeyId, TokenType);

pub struct PrivateTokenConfig {
    public: PublicTokenConfig,
    private: RwLock<HashMap<TokenType, Arc<JwtHeader>>>,
}

pub struct JwtHeader {
    encoding_key: Arc<EncodingKey>,
    header: Arc<Header>,
}

impl JwtHeader {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }
}

impl TryFrom<PublicJwkSet> for PublicTokenConfig {
    type Error = Error;
    fn try_from(set: PublicJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let token_type = jwk.metadata.token_type;

            let decoding_key = DecodingKey::from_ed_components(&jwk.metadata.x).unwrap();

            let kid: KeyId = jwk.metadata.kid;

            keys.insert((kid, token_type), Arc::new(decoding_key));
        }

        Ok(Self {
            validation: Validation::new(jsonwebtoken::Algorithm::EdDSA),
            keys: RwLock::new(keys),
        })
    }
}

impl From<PrivateJwkSet> for PublicJwkSet {
    fn from(set: PrivateJwkSet) -> Self {
        let public_keys = set
            .keys
            .into_iter()
            .map(|private_jwk| private_jwk.public)
            .collect();

        Self { keys: public_keys }
    }
}

impl TryFrom<PrivateJwkSet> for PrivateTokenConfig {
    type Error = Error;
    fn try_from(set: PrivateJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in &set.keys {
            let token_type = jwk.public.metadata.token_type;

            let kid: KeyId = jwk.public.metadata.kid;

            let encoding_key = EncodingKey::try_from(jwk.clone()).unwrap();

            let mut header = Header::new(jsonwebtoken::Algorithm::EdDSA);

            header.kid = Some(kid.into());

            let jwt_header = JwtHeader {
                encoding_key: Arc::new(encoding_key),
                header: Arc::new(header),
            };

            keys.insert(token_type, Arc::new(jwt_header));
        }

        let public_set = PublicJwkSet::from(set);

        let public_config = PublicTokenConfig::try_from(public_set)?;

        Ok(Self {
            private: RwLock::new(keys),
            public: public_config,
        })
    }
}
