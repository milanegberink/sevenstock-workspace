pub type Result<T> = core::result::Result<T, Error>;
use std::borrow::Cow;

use lib_auth::{pwd, token};

use sqlx::error::DatabaseError;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

use crate::model::store::dbx;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Entity not found: {entity} ({id})")]
    EntityNotFound { entity: &'static str, id: Uuid },

    #[error("Failed to create modelmanager")]
    CantCreateModelManagerProvider(String),

    #[error("Limited exceeded with max: {max} and actual: {actual}")]
    ListLimitOverMax { max: i64, actual: i64 },

    // -- Modules
    #[error(transparent)]
    Token(#[from] token::Error),

    #[error(transparent)]
    Pwd(#[from] pwd::Error),

    #[error("Unique violation at {table} with {constraint}")]
    UniqueViolation { table: String, constraint: String },

    #[error(r#"Username "{username}" already exists"#)]
    UserAlreadyExists { username: String },

    // -- Externals
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    Dbx(#[from] dbx::Error),

    #[error(transparent)]
    SeaQuery(#[from] sea_query::error::Error),

    #[error(transparent)]
    ModqlIntoSea(#[from] modql::filter::IntoSeaError),
}

impl Error {
    /// This function will transform the error into a more precise variant if it is an SQLX or PGError Unique Violation.
    /// The resolver can contain a function (table_name: &str, constraint: &str) that may return a specific Error if desired.
    /// If the resolver is None, or if the resolver function returns None, it will default to Error::UniqueViolation {table, constraint}.
    pub fn resolve_unique_violation<F>(self, resolver: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>,
    {
        match self
            .as_database_error()
            .map(|db_error| (db_error.code(), db_error.table(), db_error.constraint()))
        {
            // "23505" => postgresql "unique violation"
            Some((Some(Cow::Borrowed("23505")), Some(table), Some(constraint))) => resolver
                .and_then(|fun| fun(table, constraint))
                .unwrap_or_else(|| Error::UniqueViolation {
                    table: table.to_string(),
                    constraint: constraint.to_string(),
                }),
            _ => self,
        }
    }

    /// A convenient function to return the eventual database error (Postgres)
    /// if this Error is an SQLX Error that contains a database error.
    pub fn as_database_error(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Dbx(dbx::Error::Sqlx(sqlx_error)) => sqlx_error.as_database_error(),
            _ => None,
        }
    }
}
