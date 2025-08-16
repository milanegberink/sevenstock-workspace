use std::{fs, path::Path};

use ed25519_dalek::{SigningKey, VerifyingKey};
use pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rand::rngs::OsRng;

struct KeyPair {
    public_key: VerifyingKey,
    private_key: SigningKey,
}

impl KeyPair {
    fn generate() -> Self {
        let private_key = SigningKey::generate(&mut OsRng);
        let public_key = private_key.verifying_key();
        KeyPair {
            public_key,
            private_key,
        }
    }
    fn write_to_dir(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();

        fs::create_dir_all(path).expect("meow");

        let line_ending = LineEnding::LF;
        let private_path = path.join("private-key.pem");
        let public_path = path.join("public-key.pem");

        self.public_key
            .write_public_key_pem_file(public_path, line_ending)
            .expect("meow");
        self.private_key
            .write_pkcs8_pem_file(private_path, line_ending)
            .expect("meow");
    }
}

pub fn generate_new_keys() {
    [
        env!("ACCESS_TOKEN_SECRETS_DIR"),
        env!("REFRESH_TOKEN_SECRETS_DIR"),
        env!("MFA_TOKEN_SECRETS_DIR"),
    ]
    .into_iter()
    .map(|dir| KeyPair::generate().write_to_dir(dir))
    .for_each(drop);
}
