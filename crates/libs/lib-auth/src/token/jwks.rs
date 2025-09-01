use std::{collections::HashMap, hash::Hash};

use ed25519_dalek::{SigningKey, VerifyingKey};
use jsonwebtoken::{
    Algorithm,
    jwk::{
        AlgorithmParameters, CommonParameters, EllipticCurve, OctetKeyPairParameters,
        OctetKeyPairType, PublicKeyUse,
    },
};
use lib_utils::b64::b64u_encode;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

pub use jsonwebtoken::jwk::{Jwk, JwkSet};

use crate::token::KeyId;

const ALGORITHM: Algorithm = Algorithm::EdDSA;

#[derive(Serialize, Deserialize)]
pub struct SecretConfig {
    #[serde(flatten)]
    jwks: HashMap<KeyPurpose, PrivateJwk>,
}

impl Into<String> for SecretConfig {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl SecretConfig {
    pub fn new() -> Self {
        let mut jwks = HashMap::new();

        for purpose in [
            KeyPurpose::Access,
            KeyPurpose::Refresh,
            KeyPurpose::TwoFactor,
        ] {
            jwks.insert(purpose, PrivateJwk::new());
        }

        Self { jwks }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KeyPurpose {
    Access,
    Refresh,
    TwoFactor,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateJwk {
    #[serde(flatten)]
    public_jwk: Jwk,
    d: String,
}

impl PrivateJwk {
    pub fn new() -> Self {
        let (signing_key, verifying_key) = generate_key_pair();

        let d = b64u_encode(signing_key.as_bytes());
        let x = b64u_encode(verifying_key.as_bytes());

        let public_jwk = Jwk {
            common: CommonParameters {
                key_id: Some(KeyId::new().0.into()),
                key_algorithm: Some(jsonwebtoken::jwk::KeyAlgorithm::EdDSA),
                public_key_use: Some(PublicKeyUse::Signature),
                ..Default::default()
            },
            algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                key_type: OctetKeyPairType::OctetKeyPair,
                curve: EllipticCurve::Ed25519,
                x,
            }),
        };

        Self { public_jwk, d }
    }
}

fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}
