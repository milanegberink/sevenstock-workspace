use std::collections::HashMap;

use aws_config::BehaviorVersion;
use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct RotatingKeys {
    active_key: KeyPair,
    verification_keys: HashMap<KeyId, DecodingKey>,
}

pub struct KeyPair {
    kid: KeyId,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

pub struct KeyId(String);

impl RotatingKeys {
    pub async fn defaults() {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let secrets = aws_sdk_secretsmanager::Client::new(&config);
    }
}
