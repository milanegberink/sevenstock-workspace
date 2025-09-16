use aws_config::BehaviorVersion;
use aws_sdk_elasticache::Client as CacheClient;
use aws_sdk_s3::Client as S3Client;

#[derive(Clone)]
pub struct Aws {
    s3: S3Client,
    secrets: SecretsClient,
    cache: CacheClient,
}

impl Aws {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        Self {
            s3: S3Client::new(&config),
            secrets: SecretsClient::new(&config),
            cache: CacheClient::new(&config),
        }
    }
}

impl Aws {
    pub async fn s3(&self) -> &S3Client {
        &self.s3
    }
    pub fn secrets(&self) -> &SecretsClient {
        &self.secrets
    }
    pub fn cache(&self) -> &CacheClient {
        &self.cache
    }
}

pub enum Bucket {
    Public,
    Private,
}

impl Bucket {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Public => "john-burger-test-bucket",
            Self::Private => "john-burger-test-bucket",
        }
    }
}

impl Bucket {
    fn put(&self) {
        let x = Self::name(&self);
    }
}

impl From<Bucket> for String {
    fn from(b: Bucket) -> Self {
        b.name().to_string()
    }
}
