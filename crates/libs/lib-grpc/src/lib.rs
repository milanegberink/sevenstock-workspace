pub mod error;

mod proto {
    tonic::include_proto!("auth.v1");
}

pub use proto::auth_client::AuthClient;
