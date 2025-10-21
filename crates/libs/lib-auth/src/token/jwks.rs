use ed25519_dalek::SigningKey;
use jose_jwk::jose_jwa::Signing::EdDsa;
use jose_jwk::{
    Jwk, Key, Okp, OkpCurves, Parameters,
    jose_b64::serde::{Bytes, Secret},
    jose_jwa::Algorithm,
};
use rand::rngs::OsRng;

pub use jsonwebtoken::EncodingKey;
use uuid::Uuid;

pub fn generate_private_jwk() -> Jwk {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let x = Bytes::from(verifying_key.as_bytes().to_vec());
    let d = Secret::from(signing_key.as_bytes().to_vec());

    let key = Key::Okp(Okp {
        crv: OkpCurves::Ed25519,
        x,
        d: Some(d),
    });

    let prm = Parameters {
        alg: Some(Algorithm::Signing(EdDsa)),
        kid: Some(Uuid::now_v7().into()),
        ..Default::default()
    };

    let private_jwk = Jwk { key, prm };

    private_jwk
}
