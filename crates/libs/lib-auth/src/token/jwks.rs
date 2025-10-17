use ed25519_dalek::{SigningKey, VerifyingKey};
use jose_jwk::{Key, Okp};
use jsonwebtoken::{
    Algorithm,
    jwk::{EllipticCurve, OctetKeyPairType, PublicKeyUse},
};
use lib_utils::b64::{b64u_decode, b64u_encode};
use pkcs8::EncodePrivateKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::token::{Error, Result, TokenType};

pub use jsonwebtoken::EncodingKey;

// impl TryFrom<PrivateJwk> for EncodingKey {
//     type Error = Error;
//     fn try_from(jwk: PrivateJwk) -> Result<Self> {
//         let secret = jwk.d;

//         let bytes: [u8; 32] = b64u_decode(&secret).unwrap().try_into().unwrap();

//         let der = ed25519_dalek::SigningKey::from_bytes(&bytes)
//             .to_pkcs8_der()
//             .unwrap();

//         let encoding_key = EncodingKey::from_ed_der(der.as_bytes());

//         Ok(encoding_key)
//     }
// }

impl PrivateJwk {
    pub fn new(token_type: TokenType) -> Self {
        let (signing_key, verifying_key) = generate_key_pair();

        let d = b64u_encode(signing_key.as_bytes());
        let x = b64u_encode(verifying_key.as_bytes());

        let metadata = JwkMetadata {
            public_key_use: PublicKeyUse::Signature,
            alg: Algorithm::EdDSA,
            kid: Uuid::now_v7(),
            kty: OctetKeyPairType::OctetKeyPair,
            crv: EllipticCurve::Ed25519,
            x,
            token_type: token_type,
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
