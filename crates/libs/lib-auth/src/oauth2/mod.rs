use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct OAuthQuery {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: String,
}
