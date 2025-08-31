use std::collections::HashMap;

use crate::token::{Error, KeyId, Result};

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

        for purpose in [
            KeyPurpose::Access,
            KeyPurpose::Refresh,
            KeyPurpose::TwoFactor,
        ] {
            let jwks = key_store.get(&purpose).unwrap();

            for jwk in &jwks.keys {
                let decoding_key = DecodingKey::from_jwk(&jwk).unwrap();
                let kid = Uuid::parse_str(&jwk.common.key_id.as_ref().unwrap()).unwrap();

                access.insert(kid.into(), decoding_key);
            }
        }

        Ok(Self { access, refresh })
    }
}
