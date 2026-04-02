use chrono::{DateTime, Utc};

use crate::domain::value_objects::{AccountId, Money, TransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    TransferOut,
    TransferIn,
    Interest,
}

/// Immutable record of a completed financial transaction.
/// Transactions are created as a side-effect of account operations — never mutated.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    id: TransactionId,
    account_id: AccountId,
    counterparty_account_id: Option<AccountId>,
    kind: TransactionKind,
    amount: Money,
    balance_after: Money,
    description: String,
    created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        account_id: AccountId,
        counterparty_account_id: Option<AccountId>,
        kind: TransactionKind,
        amount: Money,
        balance_after: Money,
        description: String,
    ) -> Self {
        Self {
            id: TransactionId::new(),
            account_id,
            counterparty_account_id,
            kind,
            amount,
            balance_after,
            description,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn counterparty_account_id(&self) -> Option<AccountId> {
        self.counterparty_account_id
    }

    pub fn kind(&self) -> TransactionKind {
        self.kind
    }

    pub fn amount(&self) -> Money {
        self.amount
    }

    pub fn balance_after(&self) -> Money {
        self.balance_after
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
