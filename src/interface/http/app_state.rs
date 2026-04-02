use crate::application::ports::{AccountRepository, TransactionRepository};

/// Shared application state injected into all handlers via Axum's `State` extractor.
/// Generic over repository implementations — enables swapping in-memory for Postgres, etc.
#[derive(Clone)]
pub struct AppState<A: AccountRepository, T: TransactionRepository> {
    pub account_repo: A,
    pub transaction_repo: T,
}
