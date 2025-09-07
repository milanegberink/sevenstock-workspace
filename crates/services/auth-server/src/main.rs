use jsonwebtoken::{Header, encode};
use lib_auth::token::{
    TokenBuilder, TokenType,
    config::init_private_config,
    jwks::{self, EncodingKey, PrivateJwk, PrivateJwkSet},
};
use lib_grpc::{AuthServer, AuthService};
use serde::{Deserialize, Serialize};
use tonic::transport::Server;
use uuid::Uuid;
mod config;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    // let private_jwk = PrivateJwk::new(TokenType::Access);

    // println!(
    //     "{}",
    //     serde_json::to_string_pretty(&private_jwk.public).unwrap()
    // );

    // let mut set = PrivateJwkSet { keys: Vec::new() };

    // set.keys.push(private_jwk);

    // init_private_config(set).unwrap();

    // let token = TokenBuilder::access()
    //     .sub(Uuid::now_v7())
    //     .ident("Milan")
    //     .build()
    //     .await
    //     .unwrap();

    // println!("{}", token);
    //
    let x = TokenType::Access.verify("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IjAxOTkyNWQ4LTY5M2EtNzdiMC1hNGQxLTNkNThmYmEwNTY1NCJ9.eyJzdWIiOiIwMTk5MjVkOC02OTNhLTc3YjAtYTRkMS0zZDY5OGQwYzJmYmEiLCJpZGVudCI6Ik1pbGFuIiwiZXhwIjoxNzU3Mjc3NDYzLCJpYXQiOjE3NTcyNzY1NjN9.V-weM5ortZgPIoVrAwn0OYahgck7oJ_jRltwXuZAkFyDfIgi64kE43kuHClQaEkJYBeLDpiSFTODyn5vQiwRAg");

    x.await.unwrap();
    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .unwrap();
}
