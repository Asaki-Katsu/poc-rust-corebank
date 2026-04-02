use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{Account, AccountStatus};
use crate::domain::value_objects::Currency;

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub holder_name: String,
    pub currency: Currency,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub holder_name: String,
    pub balance: Decimal,
    pub currency: Currency,
    pub status: AccountStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Account> for AccountResponse {
    fn from(a: Account) -> Self {
        Self {
            id: *a.id().as_uuid(),
            holder_name: a.holder_name().to_owned(),
            balance: a.balance().amount(),
            currency: a.currency(),
            status: a.status(),
            created_at: a.created_at().to_rfc3339(),
            updated_at: a.updated_at().to_rfc3339(),
        }
    }
}
