use std::{collections::HashMap, hash::Hash, ops::Deref};

use aws_sdk_secretsmanager::Client;
use ed25519_dalek::{SigningKey, VerifyingKey};
use jsonwebtoken::{
    Algorithm, DecodingKey,
    jwk::{
        AlgorithmParameters, CommonParameters, EllipticCurve, KeyAlgorithm, OctetKeyPairParameters,
        OctetKeyPairType, PublicKeyUse,
    },
};
use lib_utils::b64::{b64u_decode, b64u_decode_der, b64u_encode};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::token::{Error, Result};

pub use jsonwebtoken::{EncodingKey, jwk::Jwk};

use crate::token::KeyId;

const ALGORITHM: Algorithm = Algorithm::EdDSA;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KeyPurpose {
    Access,
    Refresh,
    TwoFactor,
}

impl KeyPurpose {
    pub fn find(&self, kid: KeyId) {
        let id: Identifier = (kid, self.clone());
    }
}

struct Config<T>(HashMap<Identifier, T>);

pub type Identifier = (KeyId, KeyPurpose);

pub type PublicConfig = Config<DecodingKey>;

pub type PrivateConfig = Config<(EncodingKey, DecodingKey)>;

#[derive(Serialize, Deserialize)]
pub struct PublicJwk {
    #[serde(flatten)]
    inner: Jwk,
    purpose: KeyPurpose,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateJwk {
    #[serde(flatten)]
    public_jwk: PublicJwk,
    d: String,
}

#[derive(Serialize, Deserialize)]
struct JwkSet<T> {
    keys: Vec<T>,
}

pub type PrivateJwkSet = JwkSet<PrivateJwk>;

pub type PublicJwkSet = JwkSet<PublicJwk>;

impl TryFrom<PrivateJwk> for EncodingKey {
    type Error = Error;
    fn try_from(jwk: PrivateJwk) -> Result<Self> {
        let secret = jwk.d;

        let der = b64u_decode_der(&secret).unwrap();

        let encoding_key = EncodingKey::from_ed_der(&der);

        Ok(encoding_key)
    }
}

impl From<PublicJwkSet> for PublicConfig {
    fn from(set: PublicJwkSet) -> Self {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.purpose;
            let decoding_key = DecodingKey::from_jwk(&jwk.inner).unwrap();

            let kid: KeyId = jwk.inner.common.key_id.unwrap().try_into().unwrap();

            keys.insert((kid, purpose), decoding_key);
        }

        Self(keys)
    }
}

impl TryFrom<PrivateJwkSet> for PrivateConfig {
    type Error = Error;
    fn try_from(set: PrivateJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.public_jwk.purpose;
            let decoding_key = DecodingKey::from_jwk(&jwk.public_jwk.inner).unwrap();

            let kid: KeyId = jwk
                .public_jwk
                .inner
                .common
                .key_id
                .clone()
                .ok_or(Error::PrimaryKeyNotFound)?
                .try_into()?;

            let encoding_key: EncodingKey = jwk.try_into().unwrap();

            keys.insert((kid, purpose), (encoding_key, decoding_key));
        }

        Ok(Self(keys))
    }
}
impl PrivateJwk {
    pub fn new(purpose: KeyPurpose) -> Self {
        let (signing_key, verifying_key) = generate_key_pair();

        let d = b64u_encode(signing_key.as_bytes());
        let x = b64u_encode(verifying_key.as_bytes());

        let public_jwk = PublicJwk {
            inner: Jwk {
                common: CommonParameters {
                    key_id: Some(KeyId::new().0.into()),
                    key_algorithm: Some(KeyAlgorithm::EdDSA),
                    public_key_use: Some(PublicKeyUse::Signature),
                    ..Default::default()
                },
                algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                    key_type: OctetKeyPairType::OctetKeyPair,
                    curve: EllipticCurve::Ed25519,
                    x,
                }),
            },
            purpose,
        };

        Self { public_jwk, d }
    }
}

fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}
