pub type Result<T> = core::result::Result<T, Error>;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Transaction cannot be committed without an open transaction")]
    TxnCantCommitNoOpenTxn,
    #[error("Cannot begin a transaction with txn=false")]
    CannotBeginTxnWithTxnFalse,
    #[error("Cannot commit a transaction with txn=false")]
    CannotCommitTxnWithTxnFalse,
    #[error("No transaction is currently open")]
    NoTxn,

    // -- Externals
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
