use base64::DecodeError;
use base64::engine::{Engine, general_purpose};
use der::Encode;
use der::asn1::OctetString;

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>, DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(b64u)
}

pub fn b64u_decode_der(b64u: &str) -> Result<Vec<u8>, DecodeError> {
    let bytes = b64u_decode(b64u)?;

    let octec = OctetString::new(bytes).unwrap();

    let der = octec.to_der().unwrap();

    Ok(der)
}
