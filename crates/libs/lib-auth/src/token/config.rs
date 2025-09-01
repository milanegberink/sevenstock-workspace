use std::collections::HashMap;

use crate::token::{Error, KeyId, Result, jwks::PrivateJwk};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header,
    jwk::{Jwk, JwkSet},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::token::jwks::KeyPurpose;

#[derive(Serialize, Deserialize)]
pub struct KeyStore<T>(HashMap<KeyPurpose, T>);

pub struct PublicKeyConfig(HashMap<KeyPurpose, HashMap<KeyId, DecodingKey>>);

#[derive(Serialize, Deserialize)]
pub struct PrivateKeyConfig(HashMap<KeyId, PrivateJwk>);

impl From<KeyStore<JwkSet>> for PublicKeyConfig {
    fn from(ks: KeyStore<JwkSet>) -> Self {
        Self {}
    }
}

impl From<KeyStore<PrivateJwk>> for PrivateKeyConfig {
    fn from(ks: KeyStore<PrivateJwk>) -> Self {
        Self {}
    }
}

impl TokenConfig {
    pub fn from_key_store(key_store: &KeyStore<JwkSet>) -> Result<Self> {
        let mut access = HashMap::new();
        let mut refresh = HashMap::new();
        let mut two_factor = HashMap::new();

        for purpose in [
            KeyPurpose::Access,
            KeyPurpose::Refresh,
            KeyPurpose::TwoFactor,
        ] {
            let target_map = match purpose {
                KeyPurpose::Access => &mut access,
                KeyPurpose::Refresh => &mut refresh,
                KeyPurpose::TwoFactor => &mut two_factor,
            };

            let jwks = key_store.get(&purpose).unwrap();

            for jwk in &jwks.keys {
                let decoding_key = DecodingKey::from_jwk(&jwk).unwrap();
                let kid = Uuid::parse_str(&jwk.common.key_id.as_ref().unwrap()).unwrap();

                target_map.insert(kid.into(), decoding_key);
            }
        }

        Ok(Self { access, refresh })
    }
}
