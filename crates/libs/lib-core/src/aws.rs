use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_secretsmanager::Client as SecretsClient;

#[derive(Clone)]
pub struct Aws {
    s3: S3Client,
    secrets: SecretsClient,
}

impl Aws {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        Self {
            s3: S3Client::new(&config),
            secrets: SecretsClient::new(&config),
        }
    }
}

impl Aws {
    pub fn s3(&self) -> &S3Client {
        &self.s3
    }
}

pub enum Bucket {
    Public,
    Private,
}

impl Bucket {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Public => "images",
            Self::Private => "private",
        }
    }
}

impl From<Bucket> for String {
    fn from(b: Bucket) -> Self {
        b.name().to_string()
    }
}
