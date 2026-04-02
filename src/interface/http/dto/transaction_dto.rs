use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{Transaction, TransactionKind};
use crate::domain::value_objects::Currency;

#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    pub amount: Decimal,
    pub currency: Currency,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
    pub amount: Decimal,
    pub currency: Currency,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub to_account_id: Uuid,
    pub amount: Decimal,
    pub currency: Currency,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub counterparty_account_id: Option<Uuid>,
    pub kind: TransactionKind,
    pub amount: Decimal,
    pub currency: Currency,
    pub balance_after: Decimal,
    pub description: String,
    pub created_at: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            id: *tx.id().as_uuid(),
            account_id: *tx.account_id().as_uuid(),
            counterparty_account_id: tx.counterparty_account_id().map(|id| *id.as_uuid()),
            kind: tx.kind(),
            amount: tx.amount().amount(),
            currency: tx.amount().currency(),
            balance_after: tx.balance_after().amount(),
            description: tx.description().to_owned(),
            created_at: tx.created_at().to_rfc3339(),
        }
    }
}
