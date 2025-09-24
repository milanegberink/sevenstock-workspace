pub mod error;

mod proto {
    tonic::include_proto!("auth.v1");
}

use std::f64::consts::PI;

pub use crate::error::{Error, Result};
pub use crate::proto::{LoginRequest, LoginResponse};

use lib_auth::pwd::verify_password;
use lib_auth::token::{TokenBuilder, TokenType};
pub use proto::auth_client::AuthClient;
pub use proto::auth_server::AuthServer;
use tonic::Response;
pub use tonic::transport::Channel;
pub use tonic::{Request, Status};
use uuid::Uuid;

pub use crate::proto::{RefreshTokenRequest, RefreshTokenResponse, auth_server::Auth};

#[derive(Debug, Default)]
pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>> {
        let input = request.get_ref();

        let uuid = Uuid::now_v7();

        let access_token = TokenBuilder::access()
            .sub(&uuid)
            .ident("Milan/refreshed")
            .email("me@milanegberink.com")
            .avatar("https://example.com")
            .build_async()
            .await
            .map_err(Error::from)?;

        let refresh_token = TokenBuilder::refresh()
            .sub(&uuid)
            .build_async()
            .await
            .map_err(Error::from)?;

        let res = RefreshTokenResponse {
            access_token,
            refresh_token,
        };

        Ok(Response::new(res))
    }
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>> {
        let input = request.get_ref();

        let LoginRequest {
            email,
            password: pwd_bytes,
        } = input;

        verify_password("$argon2id$v=19$m=19456,t=2,p=1$CIqj9N/UsqaeW4qt6Y4dGg$PkGeH8Nj6NMO8oe1R8/WKEr14b8IL9nvYUEjVXcG8Yw".to_string(), pwd_bytes).map_err(Error::from)?;
        let uuid = Uuid::now_v7();

        let access_token = TokenBuilder::access()
            .sub(&uuid)
            .email(email)
            .ident("Milan")
            .build_async()
            .await
            .map_err(Error::from)?;

        let refresh_token = TokenBuilder::refresh()
            .sub(&uuid)
            .build_async()
            .await
            .map_err(Error::from)?;

        let res = LoginResponse {
            access_token,
            refresh_token,
        };

        Ok(Response::new(res))
    }
}
