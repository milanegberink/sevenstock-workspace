mod error;

use lib_utils::b64::b64u_encode;
use rand::{RngCore, rngs::OsRng};

use self::error::{Error, Result};

pub fn generate_api_key() -> String {
    let mut raw = [0u8; 32];
    OsRng.fill_bytes(&mut raw);
    b64u_encode(raw)
}

pub fn hash_api_key(key: &str) -> String {
    let hash = blake3::hash(key.as_bytes());
    b64u_encode(&hash.as_bytes())
}

pub fn verify_api_key(api_key: &str, stored_hash: &str) -> Result<()> {
    let candidate = blake3::hash(api_key.as_bytes());

    if b64u_encode(&candidate.as_bytes()) == stored_hash {
        Ok(())
    } else {
        Err(Error::ApiKeyInvalid)
    }
}
