pub mod error;

mod proto {
    tonic::include_proto!("auth.v1");
}

pub use proto::auth_client::AuthClient;
pub use proto::auth_server::AuthServer;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::proto::{RefreshTokenRequest, RefreshTokenResponse, auth_server::Auth};

#[derive(Debug, Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        let input = request.get_ref();

        let res = RefreshTokenResponse {
            access_token: "Hallo".to_string(),
            refresh_token: "hallo".to_string(),
        };

        Ok(Response::new(res))
    }
}
