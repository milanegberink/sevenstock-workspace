use lib_auth::token::rotating_keys::{self, RotatingSecrets, SecretManager};
use lib_grpc::{AuthServer, AuthService};
use serde::Serialize;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let mut x = SecretManager::new();
    x.rotate();
    let test = serde_json::to_string_pretty(&x).unwrap();

    println!("{}", test);

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
