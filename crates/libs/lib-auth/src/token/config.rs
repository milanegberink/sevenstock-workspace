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

pub enum Behaviour {
    Public,
    #[cfg(feature = "private")]
    Private,
}

static PUBLIC_INSTANCE: OnceLock<VerifyingConfig> = OnceLock::new();

#[cfg(feature = "private")]
static PRIVATE_INSTANCE: OnceLock<SigningConfig> = OnceLock::new();

pub fn init_config(set: PublicJwkSet) -> Result<()> {
    let public_config = VerifyingConfig::try_from(set)?;
    PUBLIC_INSTANCE
        .set(public_config)
        .map_err(|_| Error::AlreadyInitialized)?;

    Ok(())
}

pub fn init_signing_config(set: PrivateJwkSet) -> Result<()> {
    let public_config = SigningConfig::try_from(set)?;
    PRIVATE_INSTANCE
        .set(public_config)
        .map_err(|_| Error::AlreadyInitialized)?;

    Ok(())
}

pub fn verifying_config() -> &'static VerifyingConfig {
    PUBLIC_INSTANCE.get().expect("No public config initialised")
}

#[cfg(feature = "private")]
pub fn signing_config() -> &'static SigningConfig {
    PRIVATE_INSTANCE
        .get()
        .expect("No private config initialised")
}

pub struct VerifyingConfig {
    validation: Validation,
    keys: RwLock<HashMap<Identifier, Arc<DecodingKey>>>,
}

impl VerifyingConfig {
    pub async fn get(&self, id: &Identifier) -> Option<Arc<DecodingKey>> {
        let map = self.keys.read().await;
        map.get(id).cloned()
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
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
    verifying: VerifyingConfig,
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

        let public_set = PublicJwkSet::from(set);

        let public_config = VerifyingConfig::try_from(public_set)?;

        Ok(Self {
            signing: RwLock::new(keys),
            verifying: public_config,
        })
    }
}
