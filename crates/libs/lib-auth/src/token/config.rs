use std::collections::HashMap;

use crate::token::{Error, KeyId, Result};

use ed25519_dalek::ed25519::signature::digest::Key;
use jsonwebtoken::{
    Algorithm, DecodingKey, Header,
    jwk::{Jwk, JwkSet},
};
use uuid::Uuid;

use crate::token::jwks::KeyPurpose;

type KeyStore = HashMap<KeyPurpose, JwkSet>;

pub struct TokenConfig {
    access: HashMap<KeyId, DecodingKey>,
    refresh: HashMap<KeyId, DecodingKey>,
}

impl TokenConfig {
    pub fn from_key_store(key_store: &KeyStore) -> Result<Self> {
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
