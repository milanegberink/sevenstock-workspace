use crate::token::{Error, Result};
use std::{fs, path::Path};

use aws_config::BehaviorVersion;
use ed25519_dalek::{SigningKey, VerifyingKey};
use pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rand::rngs::OsRng;
use tracing::info;
use uuid::Uuid;

pub struct KeyPair {
    public_key: VerifyingKey,
    private_key: SigningKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let private_key = SigningKey::generate(&mut OsRng);
        let public_key = private_key.verifying_key();
        KeyPair {
            public_key,
            private_key,
        }
    }
    pub fn private_key_as_pem(&self) -> Result<String> {
        let pem = self.private_key.to_pkcs8_pem(LineEnding::LF)?;

        Ok(pem.to_string())
    }

    pub async fn write_to_manager(&self) -> Result<()> {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let private_key_pem = &self.private_key_as_pem()?;

        let secrets = aws_sdk_secretsmanager::Client::new(&config);

        let x = secrets
            .create_secret()
            .name(Uuid::now_v7())
            .secret_string(private_key_pem)
            .send()
            .await?;

        info!("{:?}", x);

        Ok(())
    }
}
