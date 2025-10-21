use crate::token::{Error, Result, TokenType};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use jsonwebtoken::{jwk::{Jwk, JwkSet}, DecodingKey, EncodingKey, Header, Validation};
use tokio::sync::RwLock;
use uuid::Uuid;

static PUBLIC_INSTANCE: OnceLock<VerifyingConfig> = OnceLock::new();

#[cfg(feature = "private")]
static PRIVATE_INSTANCE: OnceLock<SigningConfig> = OnceLock::new();

#[cfg(feature = "private")]
pub fn signing_config() -> Result<&'static SigningConfig> {
    PRIVATE_INSTANCE.get().ok_or(Error::NoConfigFound)
}

pub struct VerifyingConfig {
    validation: Validation,
    keys: HashMap<Uuid, DecodingKey>,
}

pub struct SigningConfig {
    encoding_key: EncodingKey,
    header: Header
}

impl VerifyingConfig {
    pub fn init(set: JwkSet) -> Result<()> {
        let public_config = VerifyingConfig::try_from(set)?;
        PUBLIC_INSTANCE
            .set(public_config)
            .map_err(|_| Error::AlreadyInitialized)?;

        Ok(())
    }
    pub fn get() -> Result<&'static Self> {
        PUBLIC_INSTANCE.get().ok_or(Error::NoConfigFound)
    }
    pub async fn get_decoding_key(&self, id: Uuid) -> Result<Arc<DecodingKey>> {
        let map = self.keys.read().await;
        let id = map
            .get(&id)
            .cloned()
            .ok_or(Error::NoDecodingCode(id))?;

        Ok(id)
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
}

impl SigningConfig {
    pub fn init(set: JwkSet) -> Result<()> {
        let private_config = SigningConfig::try_from(set)?;
        PRIVATE_INSTANCE
            .set(private_config)
            .map_err(|_| Error::AlreadyInitialized)?;

        Ok(())
    }
    pub fn get() -> Result<&'static Self> {
        PRIVATE_INSTANCE.get().ok_or(Error::NoConfigFound)
    }
    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
}



impl TryFrom<JwkSet> for VerifyingConfig {
    type Error = Error;
    fn try_from(set: JwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let decoding_key = DecodingKey::from_jwk(&jwk)?;

            let kid_str = jwk.common.key_id.ok_or(Error::InvalidJwkSet)?;

            let kid = Uuid::parse_str(&kid_str).map_err(|_| Error::InvalidToken)?;

            keys.insert(kid, Arc::new(decoding_key));
        }

        Ok(Self {
            validation: Validation::new(jsonwebtoken::Algorithm::EdDSA),
            keys: RwLock::new(keys),
        })
    }
}

impl TryFrom<Jwk> for SigningConfig {
    type Error = Error;
    fn try_from(set: Jwk) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let encoding_key = EncodingKey::from

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
