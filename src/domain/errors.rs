use rust_decimal::Decimal;
use thiserror::Error;

use super::value_objects::{AccountId, Currency};

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("negative monetary amount is not allowed")]
    NegativeAmount,

    #[error("currency mismatch: expected {expected}, got {got}")]
    CurrencyMismatch { expected: Currency, got: Currency },

    #[error("insufficient funds: available {available}, requested {requested}")]
    InsufficientFunds {
        available: Decimal,
        requested: Decimal,
    },

    #[error("account not found: {0}")]
    AccountNotFound(AccountId),

    #[error("account is frozen and cannot transact")]
    AccountFrozen,

    #[error("account holder name cannot be empty")]
    EmptyHolderName,

    #[error("duplicate account: {0}")]
    DuplicateAccount(AccountId),

    #[error("transfer to same account is not allowed")]
    SelfTransfer,
}
