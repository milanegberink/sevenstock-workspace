use lib_auth::token::jwks::{self, EncodingKey, PrivateJwk};
use lib_grpc::{AuthServer, AuthService};
use tonic::transport::Server;
mod config;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let sm = PrivateJwk::new(jwks::KeyPurpose::Access);

    println!("{}", serde_json::to_string_pretty(&sm).unwrap());

    let encoding_key: EncodingKey = sm.try_into().unwrap();

    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
