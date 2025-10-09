use lib_grpc::{AuthClient, Channel};
use moka::future::Cache;
use secrecy::SecretString;
use uuid::Uuid;

#[derive(Clone)]
pub struct Services {
    auth_client: AuthClient<Channel>,
    cache: Cache<Uuid, SecretString>,
}

impl Services {
    pub fn auth(&self) -> AuthClient<Channel> {
        self.auth_client.clone()
    }
    pub fn cache(&self) -> &Cache<Uuid, SecretString> {
        &self.cache
    }
}
