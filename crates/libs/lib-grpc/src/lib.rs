pub mod error;

mod proto {
    tonic::include_proto!("auth.v1");
}

pub use crate::error::{Error, Result};
use crate::proto::SignUpResponse;
pub use crate::proto::{LoginRequest, LoginResponse};
use lib_auth::pwd::verify_password;
use lib_auth::token::{TokenBuilder, TokenType};
use lib_core::ctx::Ctx;
use lib_core::model::permission::{Action, Resource};
use lib_core::model::user::{UserBmc, UserForCreate, UserForLogin};
pub use proto::auth_client::AuthClient;
pub use proto::auth_server::AuthServer;
use secrecy::SecretString;
use tonic::Response;
pub use tonic::transport::Channel;
pub use tonic::{Request, Status};
use uuid::Uuid;

pub use crate::proto::{
    RefreshTokenRequest, RefreshTokenResponse, SignUpRequest, auth_server::Auth,
};
use lib_core::model::ModelManager;

pub struct AuthService {
    mm: ModelManager,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>> {
        let input = request.get_ref();

        let RefreshTokenRequest { refresh_token } = input;

        TokenType::Refresh
            .verify(&refresh_token)
            .await
            .map_err(Error::from)?;

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

        let LoginRequest { email, pwd_clear } = input;

        let ctx = Ctx::root_ctx();

        let user: UserForLogin = UserBmc::first_by_email(&ctx, &self.mm, &email)
            .await
            .map_err(Error::from)?;

        let Some(pwd) = user.pwd else {
            return Err(Status::unauthenticated("C"));
        };

        verify_password(&pwd, &pwd_clear).map_err(Error::from)?;

        let access_token = TokenBuilder::access()
            .sub(&user.id)
            .email(email)
            .ident(user.username)
            .scope(Permission::new(Resource::Product, Level::All))
            .build_async()
            .await
            .map_err(Error::from)?;

        let refresh_token = TokenBuilder::refresh()
            .sub(&user.id)
            .build_async()
            .await
            .map_err(Error::from)?;

        let res = LoginResponse {
            access_token,
            refresh_token,
        };

        Ok(Response::new(res))
    }

    async fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>> {
        let SignUpRequest {
            username,
            pwd_clear,
            email,
        } = request.get_ref().clone();

        let user_c = UserForCreate {
            username,
            email,
            pwd_clear: SecretString::new(pwd_clear.into_boxed_str()),
        };

        let id = UserBmc::create(&Ctx::root_ctx(), &self.mm, user_c)
            .await
            .map_err(Error::from)?;

        let res = Response::new(SignUpResponse { id: id.into() });

        Ok(res)
    }
}
