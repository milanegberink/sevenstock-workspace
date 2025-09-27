use lib_auth::token::{
    TokenType,
    config::{init_signing_config, init_verifying_config},
    jwks::{PrivateJwk, PrivateJwkSet, PublicJwkSet},
};
use lib_grpc::{AuthServer, AuthService};
use tonic::transport::Server;
mod config;

#[tokio::main]
async fn main() {
    let private_jwk = PrivateJwk::new(TokenType::Access);

    let private_jwk_2 = PrivateJwk::new(TokenType::Refresh);

    let mut set = PrivateJwkSet { keys: Vec::new() };

    set.keys.push(private_jwk);

    set.keys.push(private_jwk_2);

    let public_set = PublicJwkSet::from(set.clone());

    init_signing_config(set).unwrap();

    init_verifying_config(public_set).unwrap();

    let addr = "[::]:50051".parse().unwrap();

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
