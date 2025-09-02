use std::collections::HashMap;

use crate::token::{Error, KeyId, Result, jwks::PrivateJwk};

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header,
    jwk::{Jwk, JwkSet},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::token::jwks::KeyPurpose;

#[derive(Serialize, Deserialize)]
pub struct KeyStore<T>(HashMap<KeyPurpose, T>);

pub struct PublicKeyConfig(HashMap<KeyPurpose, HashMap<KeyId, DecodingKey>>);

#[derive(Serialize, Deserialize)]
pub struct PrivateKeyConfig(HashMap<KeyId, PrivateJwk>);

// impl From<KeyStore<JwkSet>> for PublicKeyConfig {
//     fn from(ks: KeyStore<JwkSet>) -> Self {
//         Self {}
//     }
// }

// impl From<KeyStore<PrivateJwk>> for PrivateKeyConfig {
//     fn from(ks: KeyStore<PrivateJwk>) -> Self {
//         Self {}
//     }
// }
