use ed25519_dalek::{SigningKey, VerifyingKey};
use jsonwebtoken::{
    Algorithm,
    jwk::{EllipticCurve, OctetKeyPairType, PublicKeyUse},
};
use lib_utils::b64::{b64u_decode_der, b64u_encode};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::token::{Error, Result, TokenPurpose};

pub use jsonwebtoken::EncodingKey;

use crate::token::KeyId;

#[derive(Serialize, Deserialize)]
pub struct JwkSet<T> {
    pub keys: Vec<T>,
}

pub type PrivateJwkSet = JwkSet<PrivateJwk>;

pub type PublicJwkSet = JwkSet<PublicJwk>;

#[derive(Serialize, Deserialize)]
pub struct JwkMetadata {
    #[serde(rename = "use")]
    pub public_key_use: PublicKeyUse,
    pub purpose: TokenPurpose,
    pub kid: KeyId,
    pub x: String,
    alg: Algorithm,
    kty: OctetKeyPairType,
    crv: EllipticCurve,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateJwk {
    #[serde(flatten)]
    pub public: PublicJwk,
    pub d: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublicJwk {
    #[serde(flatten)]
    pub metadata: JwkMetadata,
}

#[derive(Serialize, Deserialize)]
pub enum Jwk {
    Public(PublicJwk),
    Private(PrivateJwk),
}

impl TryFrom<PrivateJwk> for EncodingKey {
    type Error = Error;
    fn try_from(jwk: PrivateJwk) -> Result<Self> {
        let secret = jwk.d;

        let der = b64u_decode_der(&secret).unwrap();

        let encoding_key = EncodingKey::from_ed_der(&der);

        Ok(encoding_key)
    }
}

impl PrivateJwk {
    pub fn new(purpose: TokenPurpose) -> Self {
        let (signing_key, verifying_key) = generate_key_pair();

        let d = b64u_encode(signing_key.as_bytes());
        let x = b64u_encode(verifying_key.as_bytes());

        let metadata = JwkMetadata {
            public_key_use: PublicKeyUse::Signature,
            alg: Algorithm::EdDSA,
            kid: KeyId::new(),
            kty: OctetKeyPairType::OctetKeyPair,
            crv: EllipticCurve::Ed25519,
            x,
            purpose,
        };

        let public = PublicJwk { metadata };

        Self { public, d }
    }
}

fn generate_key_pair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}
