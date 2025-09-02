use std::{collections::HashMap, hash::Hash};

use aws_sdk_secretsmanager::Client;
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

pub use jsonwebtoken::jwk::Jwk;

use crate::token::KeyId;

const ALGORITHM: Algorithm = Algorithm::EdDSA;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KeyPurpose {
    Access,
    Refresh,
    TwoFactor,
}

#[derive(Serialize, Deserialize)]
pub struct PublicJwk {
    #[serde(flatten)]
    jwk: Jwk,
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
    set: Vec<T>,
}

pub type PrivateJwkSet = JwkSet<PrivateJwk>;

pub type PublicJwkSet = JwkSet<PublicJwk>;

impl PrivateJwkSet {}

impl From<Client> for PrivateJwkSet {
    fn from(client: Client) -> Self {}
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
