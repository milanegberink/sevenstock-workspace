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

static PUBLIC_INSTANCE: OnceLock<VerifyingConfig> = OnceLock::new();

#[cfg(feature = "private")]
static PRIVATE_INSTANCE: OnceLock<SigningConfig> = OnceLock::new();

#[cfg(feature = "private")]
pub fn signing_config() -> Result<&'static SigningConfig> {
    PRIVATE_INSTANCE.get().ok_or(Error::NoConfigFound)
}

pub struct VerifyingConfig {
    validation: Validation,
    keys: RwLock<HashMap<Identifier, Arc<DecodingKey>>>,
    jwk_set: PublicJwkSet,
}

impl VerifyingConfig {
    pub fn init(set: PublicJwkSet) -> Result<()> {
        let public_config = VerifyingConfig::try_from(set)?;
        PUBLIC_INSTANCE
            .set(public_config)
            .map_err(|_| Error::AlreadyInitialized)?;

        Ok(())
    }
    pub fn get() -> Result<&'static Self> {
        PUBLIC_INSTANCE.get().ok_or(Error::NoConfigFound)
    }
    pub async fn get_decoding_key(&self, id: &Identifier) -> Result<Arc<DecodingKey>> {
        let map = self.keys.read().await;
        map.get(id)
            .cloned()
            .ok_or(Error::NoDecodingCode(id.0.into()))
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
    pub fn pub_jwk_set(&self) -> &PublicJwkSet {
        &self.jwk_set
    }
}

impl SigningConfig {
    pub fn init(set: PrivateJwkSet) -> Result<()> {
        let private_config = SigningConfig::try_from(set)?;
        PRIVATE_INSTANCE
            .set(private_config)
            .map_err(|_| Error::AlreadyInitialized)?;

        Ok(())
    }
    pub async fn get(&self, token_type: TokenType) -> Option<Arc<JwtHeader>> {
        let map = self.signing.read().await;
        map.get(&token_type).cloned()
    }
}

pub type Identifier = (KeyId, TokenType);

pub struct SigningConfig {
    signing: RwLock<HashMap<TokenType, Arc<JwtHeader>>>,
}

pub struct JwtHeader {
    encoding_key: Arc<EncodingKey>,
    header: Arc<Header>,
}

impl JwtHeader {
    pub(crate) fn header(&self) -> &Header {
        &self.header
    }
    pub(crate) fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }
}

impl TryFrom<PublicJwkSet> for VerifyingConfig {
    type Error = Error;
    fn try_from(set: PublicJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in &set.keys {
            let token_type = jwk.metadata.token_type;

            let decoding_key = DecodingKey::from_ed_components(&jwk.metadata.x).unwrap();

            let kid: KeyId = jwk.metadata.kid;

            keys.insert((kid, token_type), Arc::new(decoding_key));
        }

        Ok(Self {
            validation: Validation::new(jsonwebtoken::Algorithm::EdDSA),
            keys: RwLock::new(keys),
            jwk_set: set,
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

impl TryFrom<PrivateJwkSet> for SigningConfig {
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

        Ok(Self {
            signing: RwLock::new(keys),
        })
    }
}
