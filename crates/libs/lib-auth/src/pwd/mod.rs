mod config;
mod error;

use secrecy::{ExposeSecret, SecretBox, SecretString};

use crate::pwd::config::pwd_config;

use argon2::password_hash::{
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
};

pub use self::error::{Error, Result};

pub fn hash_password(password: SecretString) -> Result<String> {
    let argon2 = pwd_config().argon2();

    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map_err(|_| Error::PasswordHashFail)?
        .to_string();

    Ok(hash)
}

pub fn verify_password(pw_hash: &str, pwd: &str) -> Result<()> {
    let argon2 = pwd_config().argon2();
    let parsed_hash = PasswordHash::new(&pw_hash).map_err(|_| Error::PasswordHashFail)?;

    let ver_res = argon2
        .verify_password(pwd.as_bytes(), &parsed_hash)
        .map_err(|_| Error::PasswordHashFail);

    ver_res
}
