use base64::DecodeError;
use base64::engine::{Engine, general_purpose};

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>, DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(b64u)
}
