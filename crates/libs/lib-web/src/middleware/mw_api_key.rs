use axum::http::HeaderValue;
use axum_extra::headers::authorization::Credentials;

pub struct ApiKey(pub String);

impl Credentials for ApiKey {
    const SCHEME: &'static str = "ApiKey";

    fn encode(&self) -> HeaderValue {
        let value = format!("{} {}", Self::SCHEME, self.0);
        HeaderValue::from_str(&value).expect("Invalid header value")
    }

    fn decode(value: &HeaderValue) -> Option<Self> {
        let s = value.to_str().ok()?;
        s.strip_prefix(&(Self::SCHEME.to_string() + " "))
            .map(|key| ApiKey(key.to_string()))
    }
}
