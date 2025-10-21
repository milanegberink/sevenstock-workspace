mod error;

use blake3::Hash;
use lib_utils::b64::b64u_encode;
use rand::{RngCore, rngs::OsRng};
use secrecy::{ExposeSecret, SecretString};

use self::error::{Error, Result};

pub fn generate_secret_key_b64u() -> SecretString {
    let mut raw = [0u8; 32];
    OsRng.fill_bytes(&mut raw);
    let secret = b64u_encode(raw);
    SecretString::new(secret.into_boxed_str())
}

pub fn hash_secret_key(key: &str) -> [u8; 32] {
    *blake3::hash(key.as_bytes()).as_bytes()
}

pub fn verify_secret_key(api_key: &str, stored_hash: &str) -> Result<()> {
    let candidate = blake3::hash(api_key.as_bytes());
    let candidate_b64 = b64u_encode(&candidate.as_bytes());

    if candidate_b64 == stored_hash {
        Ok(())
    } else {
        Err(Error::ApiKeyInvalid)
    }
}
