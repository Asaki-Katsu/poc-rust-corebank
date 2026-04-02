use crate::application::ports::{AccountRepositoryError, TransactionRepositoryError};
use crate::domain::errors::DomainError;

/// Unified application-level error that maps domain and infrastructure failures.
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("{0}")]
    Domain(#[from] DomainError),

    #[error("account repository: {0}")]
    AccountRepo(#[from] AccountRepositoryError),

    #[error("transaction repository: {0}")]
    TransactionRepo(#[from] TransactionRepositoryError),
}
