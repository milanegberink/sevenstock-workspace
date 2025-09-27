use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| CoreConfig::load_from_env())
}

pub struct CoreConfig {
    db_url: String,
    redis_url: String,
    auth_grpc: String,
}

impl CoreConfig {
    pub fn db_url(&self) -> &str {
        &self.db_url
    }
    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }
    pub fn auth_grpc(&self) -> &str {
        &self.auth_grpc
    }
}

impl CoreConfig {
    fn load_from_env() -> Self {
        Self {
            db_url: std::env::var("DATABASE_URL").expect("No database URL found"),
            redis_url: std::env::var("REDIS_URL").expect("No redis URL found"),
            auth_grpc: std::env::var("AUTH_GRPC_URL").expect("No redis URL found"),
        }
    }
}
