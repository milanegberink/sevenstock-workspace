use thiserror::Error;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to load proto files")]
    ProtoLoadFail,
}
