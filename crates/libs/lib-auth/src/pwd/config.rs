use std::sync::OnceLock;

use argon2::Argon2;

pub struct PwdConfig {
    argon2: Argon2<'static>,
}

pub fn pwd_config() -> &'static PwdConfig {
    static INSTANCE: OnceLock<PwdConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| PwdConfig::new())
}

impl PwdConfig {
    fn new() -> Self {
        let argon2 = Argon2::default();
        let config = Self { argon2 };
        config
    }
    pub fn argon2(&self) -> &Argon2<'static> {
        &self.argon2
    }
}
