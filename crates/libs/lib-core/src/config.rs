use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| CoreConfig::load_from_env())
}

pub struct CoreConfig {
    db_url: &'static str,
    redis_url: &'static str,
}

impl CoreConfig {
    pub fn db_url(&self) -> &str {
        &self.db_url
    }
    pub fn redis_url(&self) -> &str {
        &self.redis_url
    }
}

impl CoreConfig {
    fn load_from_env() -> Self {
        Self {
            db_url: env!("DATABASE_URL"),
            redis_url: env!("REDIS_URL"),
        }
    }
}
