use crate::token::{Error, Result, TokenPurpose};
use std::{collections::HashMap, sync::{Arc, OnceLock}};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use tokio::sync::RwLock;

use crate::token::{
    KeyId,
    jwks::{PrivateJwkSet, PublicJwkSet},
};


static PUBLIC_INSTANCE: OnceLock<PublicTokenConfig> = OnceLock::new();
static PRIVATE_INSTANCE: OnceLock<PrivateTokenConfig> = OnceLock::new();

pub fn public_config() -> &'static PublicTokenConfig {
    PUBLIC_INSTANCE.get_or_init(|| PublicTokenConfig::new())
}

pub fn private_config() -> &'static PrivateTokenConfig {
    PRIVATE_INSTANCE.get_or_init(|| PrivateTokenConfig::new())
}

pub struct PublicTokenConfig {
    validation: Validation,
    keys: RwLock<HashMap<Identifier, Arc<DecodingKey>>>,
}

impl PublicTokenConfig {
    pub async fn get(&self, id: &Identifier) -> Option<Arc<DecodingKey>>
    {
        let map = self.keys.read().await;
        map.get(id).cloned()
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
}

impl PrivateTokenConfig {
    pub async fn get(&self, purpose: TokenPurpose) -> Option<Arc<Primary>>
    {
        let map = self.private.read().await;
        map.get(&purpose).cloned()
    }
}

pub type Identifier = (KeyId, TokenPurpose);

struct PrivateTokenConfig {
    public: PublicTokenConfig,
    private: RwLock<HashMap<TokenPurpose, Arc<Primary>>>,
}

struct Primary {
    encoding_key: Arc<EncodingKey>,
    header: Arc<Header>,
}

impl Primary {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }
}

impl TryFrom<PublicJwkSet> for PublicTokenConfig {
    type Error = Error;
    fn try_from(set: PublicJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.metadata.purpose;

            let decoding_key = DecodingKey::from_ed_components(&jwk.metadata.x).unwrap();

            let kid: KeyId = jwk.metadata.kid;



            keys.insert((kid, purpose), decoding_key);
        }

        Ok(Self {
            validation: Validation::new(jsonwebtoken::Algorithm::EdDSA),
            keys: RwLock::new(keys),
        })
    }
}

impl TryFrom<PrivateJwkSet> for PrivateTokenConfig {
    type Error = Error;
    fn try_from(set: PrivateJwkSet) -> Result<Self> {
        let mut keys = HashMap::new();
        for jwk in set.keys {
            let purpose = jwk.public.metadata.purpose;
            let decoding_key = DecodingKey::from_ed_components(&jwk.d).unwrap();

            let kid: KeyId = jwk.public.metadata.kid;

            let encoding_key = EncodingKey::try_from(jwk).unwrap();

            let header = Header::new(jsonwebtoken::Algorithm::EdDSA);

            header.kid = Some(kid.into());

            let primary = Primary {
                encoding_key: Arc::new(encoding_key),
                header: Arc::new(header),
            };

            keys.insert(purpose, primary);
        }

        Ok(Self {
            private: RwLock::new(keys),
            public:
        })
    }
}
