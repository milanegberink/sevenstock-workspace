use lib_auth::token::jwks::{self, PrivateJwk, SecretConfig};
use lib_grpc::{AuthServer, AuthService};
use tonic::transport::Server;
mod config;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let sm = SecretConfig::new();

    println!("{}", serde_json::to_string_pretty(&sm).unwrap());

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
