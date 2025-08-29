use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::token::{Error, Result};
use ed25519_dalek::SigningKey;
use jsonwebtoken::get_current_timestamp;
use pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as, skip_serializing_none};
use uuid::Uuid;
use zeroize::{Zeroize, Zeroizing};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct KeyId(Uuid);

#[derive(Serialize, Deserialize)]
pub struct SecretManager {
    secrets: HashMap<KeyPurpose, RotatingSecrets>,
}

impl SecretManager {
    pub fn rotate(&mut self) -> Result<()> {
        self.secrets
            .get_mut(&KeyPurpose::Access)
            .unwrap()
            .rotate()?;
        self.secrets
            .get_mut(&KeyPurpose::Refresh)
            .unwrap()
            .rotate()?;
        self.secrets
            .get_mut(&KeyPurpose::TwoFactor)
            .unwrap()
            .rotate()?;

        Ok(())
    }
    pub fn new() -> Self {
        let mut secrets = HashMap::new();

        secrets.insert(KeyPurpose::Access, RotatingSecrets::new());
        secrets.insert(KeyPurpose::Refresh, RotatingSecrets::new());
        secrets.insert(KeyPurpose::TwoFactor, RotatingSecrets::new());

        Self { secrets }
    }
}

impl FromStr for SecretManager {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let sm = serde_json::from_str(s)?;
        Ok(sm)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RotatingSecrets {
    active_kid: KeyId,
    secrets: HashMap<KeyId, Key>,
}

impl RotatingSecrets {
    fn rotate(&mut self) -> Result<()> {
        let kid = KeyId::generate();
        let key = Key::new(kid.clone())?;

        let active_key = self.secrets.get_mut(&self.active_kid).unwrap();

        active_key.private_secret.zeroize();

        active_key.status = SecretStatus::Verifying;

        self.secrets.retain(|_id, key| !key.is_expired());

        self.secrets.insert(kid.clone(), key);

        self.active_kid = kid;

        Ok(())
    }
    pub fn new() -> Self {
        let kid = KeyId::generate();

        let mut secrets = HashMap::new();

        secrets.insert(kid.clone(), Key::new(kid.clone()).unwrap());
        Self {
            active_kid: kid,
            secrets,
        }
    }
}

impl KeyId {
    fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum KeyPurpose {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "refresh")]
    Refresh,
    #[serde(rename = "two-factor")]
    TwoFactor,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Key {
    kid: KeyId,
    private_secret: Option<Zeroizing<String>>,
    public_secret: String,
    #[serde_as(as = "DisplayFromStr")]
    iat: u64,
    #[serde_as(as = "DisplayFromStr")]
    exp: u64,
    status: SecretStatus,
}

impl Key {
    fn new(kid: KeyId) -> Result<Self> {
        let line_ending = LineEnding::LF;

        let signign_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signign_key.verifying_key();

        let private_secret = signign_key.to_pkcs8_pem(line_ending)?;
        let public_secret = verifying_key.to_public_key_pem(line_ending)?;

        let timestamp = get_current_timestamp();
        Ok(Self {
            kid,
            private_secret: Some(private_secret),
            public_secret,
            iat: timestamp,
            exp: timestamp + 10000,
            status: SecretStatus::Active,
        })
    }
    fn is_expired(&self) -> bool {
        get_current_timestamp() > self.exp
    }
}

#[derive(Serialize, Deserialize)]
pub enum SecretStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "verifying")]
    Verifying,
}
