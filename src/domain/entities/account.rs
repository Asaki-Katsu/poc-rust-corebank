use chrono::{DateTime, Utc};

use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AccountId, Currency, Money};

/// Account status with explicit state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

/// Core domain entity representing a bank account.
/// All balance mutations go through domain methods that enforce invariants.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    id: AccountId,
    holder_name: String,
    balance: Money,
    status: AccountStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Account {
    pub fn open(holder_name: String, currency: Currency) -> Result<Self, DomainError> {
        if holder_name.trim().is_empty() {
            return Err(DomainError::EmptyHolderName);
        }
        let now = Utc::now();
        Ok(Self {
            id: AccountId::new(),
            holder_name: holder_name.trim().to_owned(),
            balance: Money::zero(currency),
            status: AccountStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence — skips business rules.
    pub fn reconstitute(
        id: AccountId,
        holder_name: String,
        balance: Money,
        status: AccountStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            holder_name,
            balance,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Accessors ---

    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn holder_name(&self) -> &str {
        &self.holder_name
    }

    pub fn balance(&self) -> Money {
        self.balance
    }

    pub fn currency(&self) -> Currency {
        self.balance.currency()
    }

    pub fn status(&self) -> AccountStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behaviour ---

    fn ensure_active(&self) -> Result<(), DomainError> {
        if self.status != AccountStatus::Active {
            return Err(DomainError::AccountFrozen);
        }
        Ok(())
    }

    pub fn deposit(&mut self, amount: Money) -> Result<(), DomainError> {
        self.ensure_active()?;
        self.balance = (self.balance + amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<(), DomainError> {
        self.ensure_active()?;
        self.balance = (self.balance - amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn freeze(&mut self) {
        self.status = AccountStatus::Frozen;
        self.updated_at = Utc::now();
    }

    pub fn unfreeze(&mut self) {
        self.status = AccountStatus::Active;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn open_account_with_zero_balance() {
        let acc = Account::open("Alice".into(), Currency::THB).unwrap();
        assert!(acc.balance().is_zero());
        assert_eq!(acc.status(), AccountStatus::Active);
    }

    #[test]
    fn rejects_empty_holder_name() {
        assert!(Account::open("  ".into(), Currency::THB).is_err());
    }

    #[test]
    fn deposit_and_withdraw() {
        let mut acc = Account::open("Bob".into(), Currency::USD).unwrap();
        let hundred = Money::new(dec!(100), Currency::USD).unwrap();
        let thirty = Money::new(dec!(30), Currency::USD).unwrap();

        acc.deposit(hundred).unwrap();
        assert_eq!(acc.balance().amount(), dec!(100));

        acc.withdraw(thirty).unwrap();
        assert_eq!(acc.balance().amount(), dec!(70));
    }

    #[test]
    fn frozen_account_cannot_transact() {
        let mut acc = Account::open("Charlie".into(), Currency::THB).unwrap();
        acc.freeze();
        let amount = Money::new(dec!(10), Currency::THB).unwrap();
        assert!(acc.deposit(amount).is_err());
    }
}
