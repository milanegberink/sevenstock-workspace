use lib_auth::token::{
    TokenBuilder, TokenType,
    config::init_signing_config,
    jwks::{PrivateJwk, PrivateJwkSet},
};
use lib_grpc::{AuthServer, AuthService};
use tonic::transport::Server;
use uuid::Uuid;
mod config;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let private_jwk = PrivateJwk::new(TokenType::Access);

    println!(
        "{}",
        serde_json::to_string_pretty(&private_jwk.public).unwrap()
    );

    let mut set = PrivateJwkSet { keys: Vec::new() };

    set.keys.push(private_jwk);

    init_signing_config(set).unwrap();

    let token = TokenBuilder::access()
        .sub(Uuid::now_v7())
        .ident("Milan")
        .email("me@milanegberink.com")
        .avatar("https://example.com")
        .build_async()
        .await
        .unwrap();

    println!("{}", token);

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
