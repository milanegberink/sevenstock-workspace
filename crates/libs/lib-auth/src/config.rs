// use argon2::Argon2;
// use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
// use std::{collections::HashMap, sync::OnceLock};
// use tokio::sync::RwLock;

// pub fn auth_config() -> &'static AuthConfig {
//     static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

//     INSTANCE.get_or_init(|| AuthConfig::load_from_env())
// }

// pub struct KeyPairs {
//     access: RotatingKeys,
//     refresh: RotatingKeys,
//     auth_2fa: RotatingKeys,
// }

// pub struct AuthConfig {
//     keys: RwLock<KeyPairs>,
//     argon2: Argon2<'static>,
//     jwt_ctx: JwtContext,
// }

// impl KeyPairs {
//     pub(crate) fn access(&self) -> &KeyPair {
//         &self.access
//     }
//     pub(crate) fn refresh(&self) -> &KeyPair {
//         &self.refresh
//     }
//     pub(crate) fn auth_2fa(&self) -> &KeyPair {
//         &self.auth_2fa
//     }
// }

// impl AuthConfig {
//     pub(crate) fn argon2(&self) -> &Argon2 {
//         &self.argon2
//     }
//     pub(crate) fn jwt_ctx(&self) -> &JwtContext {
//         &self.jwt_ctx
//     }
//     pub(crate) fn keys(&self) -> &KeyPairs {
//         &self.keys
//     }
// }

// pub struct KeyPair {
//     kid: KeyId,
//     encoding: EncodingKey,
//     decoding: DecodingKey,
// }

// pub struct KeyId(String);

// impl KeyPair {
//     pub fn encoding(&self) -> &EncodingKey {
//         &self.encoding
//     }
//     pub fn decoding(&self) -> &DecodingKey {
//         &self.decoding
//     }
// }

// impl KeyPair {
//     async fn defaults(secrets_manager: &Client) -> Self {
//         DecodingKey::from_

//         Self {}
//     }
//     // fn rotate(secrets_manager: &Client) -> Self {
//     //     secrets_manager.de
//     // }
// }

// pub struct JwtContext {
//     header: Header,
//     validation: Validation,
// }

// impl JwtContext {
//     fn new() -> Self {
//         let alg = Algorithm::EdDSA;

//         Self {
//             header: Header::new(alg),
//             validation: Validation::new(alg),
//         }
//     }
// }

// impl JwtContext {
//     pub fn header(&self) -> &Header {
//         &self.header
//     }
//     pub fn validation(&self) -> &Validation {
//         &self.validation
//     }
// }

// impl AuthConfig {
//     fn load_from_env() -> Self {
//         Self {
//             keys: KeyPairs {
//                 access: KeyPair::new(env!("ACCESS_TOKEN_SECRETS_DIR")),
//                 refresh: KeyPair::new(env!("REFRESH_TOKEN_SECRETS_DIR")),
//                 auth_2fa: KeyPair::new(env!("MFA_TOKEN_SECRETS_DIR")),
//             },
//             argon2: Argon2::default(),
//             jwt_ctx: JwtContext::new(),
//         }
//     }
//     fn defaults() -> Self {
//         todo!();
//     }
// }
