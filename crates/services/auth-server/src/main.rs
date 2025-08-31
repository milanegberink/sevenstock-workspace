use lib_auth::token::jwks::{self, PrivateJwk, SecretConfig, test};
use lib_grpc::{AuthServer, AuthService};
use tonic::transport::Server;
mod config;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    test();

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
