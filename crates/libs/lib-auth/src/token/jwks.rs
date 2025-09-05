use std::{collections::HashMap, hash::Hash};

use ed25519_dalek::{SigningKey, VerifyingKey};
use jsonwebtoken::{
    Algorithm, DecodingKey,
    jwk::{EllipticCurve, OctetKeyPairType, PublicKeyUse},
};
use lib_utils::b64::{b64u_decode_der, b64u_encode};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::token::{Error, Result};

pub use jsonwebtoken::EncodingKey;

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
    pub fn find(&self, kid: KeyId) -> T {
        let id: Identifier = (kid, self.clone());
    }
}

struct Store<T>(HashMap<Identifier, T>);

pub type Identifier = (KeyId, KeyPurpose);

pub type PublicStore = Store<DecodingKey>;

pub type PrivateStore = Store<(EncodingKey, DecodingKey)>;

#[derive(Serialize, Deserialize)]
struct JwkSet<T> {
    keys: Vec<T>,
}

pub type PrivateJwkSet = JwkSet<PrivateJwk>;

pub type PublicJwkSet = JwkSet<PublicJwk>;

#[derive(Serialize, Deserialize)]
pub struct JwkMetadata {
    #[serde(rename = "use")]
    public_key_use: PublicKeyUse,
    alg: Algorithm,
    kid: KeyId,
    kty: OctetKeyPairType,
    crv: EllipticCurve,
    purpose: KeyPurpose,
    x: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateJwk {
    #[serde(flatten)]
    public: PublicJwk,
    d: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublicJwk {
    #[serde(flatten)]
    metadata: JwkMetadata,
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

impl TryFrom<PublicJwkSet> for PublicStore {
    type Error = Error;
    fn try_from(set: PublicJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.metadata.purpose;

            let decoding_key = DecodingKey::from_ed_components(&jwk.metadata.x).unwrap();

            let kid: KeyId = jwk.metadata.kid;

            keys.insert((kid, purpose), decoding_key);
        }

        Ok(Self(keys))
    }
}

impl TryFrom<PrivateJwkSet> for PrivateStore {
    type Error = Error;
    fn try_from(set: PrivateJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.public.metadata.purpose;
            let decoding_key = DecodingKey::from_ed_components(&jwk.d).unwrap();

            let kid: KeyId = jwk.public.metadata.kid;

            let encoding_key = EncodingKey::try_from(jwk).unwrap();

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
