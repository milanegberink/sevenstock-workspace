use crate::token::{Error, Result, TokenType};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use ed25519_dalek::SigningKey;
use jose_jwk::{Jwk, Key};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, jwk::JwkSet};
use lib_utils::b64::b64u_decode;
use pkcs8::EncodePrivateKey;
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
    keys: HashMap<Uuid, Arc<DecodingKey>>,
}

pub struct SigningConfig {
    encoding_key: EncodingKey,
    header: Header,
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
    pub fn get_decoding_key(&self, id: Uuid) -> Result<Arc<DecodingKey>> {
        let key = self.keys.get(&id).cloned().ok_or(Error::InvalidToken)?;

        Ok(key)
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
}

impl TryFrom<Jwk> for SigningConfig {
    type Error = Error;
    fn try_from(jwk: Jwk) -> Result<Self> {
        let Key::Okp(okp_key) = &jwk.key else {
            return Err(Error::InvalidJwkSet);
        };

        let d = okp_key.d.as_ref().ok_or(Error::InvalidJwkSet)?;

        let bytes: [u8; 32] = d.as_ref().try_into().map_err(|_| Error::InvalidJwkSet)?;

        let secret = SigningKey::from_bytes(&bytes)
            .to_pkcs8_der()
            .map_err(|_| Error::InvalidToken)?;

        let encoding_key = EncodingKey::from_ed_der(secret.as_bytes());

        let mut header = Header::new(jsonwebtoken::Algorithm::EdDSA);

        let kid = jwk.prm.kid.ok_or(Error::InvalidJwkSet)?;

        header.kid = Some(kid);

        Ok(Self {
            encoding_key,
            header,
        })
    }
}

impl SigningConfig {
    pub fn init(set: Jwk) -> Result<()> {
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
            keys,
        })
    }
}
