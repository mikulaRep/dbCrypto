use rusqlite::Error as RusqliteError;
use std::env::VarError;
use std::io::Error as IoError;
use std::num::ParseIntError;
use thiserror::Error;

pub type CryptoResult<T> = Result<T, CryptoError>;

#[derive(Debug, Error)]
pub enum CryptoError {
    // Rusqlite
    #[error("{0}")]
    Rusqlite(#[from] RusqliteError),

    // Std
    #[error("{0}")]
    Io(#[from] IoError),
    #[error("{0}")]
    ParseInt(#[from] ParseIntError),
    #[error("{0}")]
    Var(#[from] VarError),

    // Generic
    #[error("{0}")]
    GenericErr(String),
}

impl CryptoError {
    pub fn generic_err(msg: impl Into<String>) -> Self {
        CryptoError::GenericErr(msg.into())
    }
}
