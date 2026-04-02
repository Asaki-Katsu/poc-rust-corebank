use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

use crate::domain::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Currency {
    THB,
    USD,
    EUR,
    GBP,
    JPY,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::THB => write!(f, "THB"),
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::JPY => write!(f, "JPY"),
        }
    }
}

/// Value object representing a monetary amount with currency.
/// Enforces non-negative amounts and same-currency arithmetic at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, DomainError> {
        if amount < Decimal::ZERO {
            return Err(DomainError::NegativeAmount);
        }
        Ok(Self { amount, currency })
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    pub fn checked_sub(self, other: Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::CurrencyMismatch {
                expected: self.currency,
                got: other.currency,
            });
        }
        let result = self.amount - other.amount;
        if result < Decimal::ZERO {
            return Err(DomainError::InsufficientFunds {
                available: self.amount,
                requested: other.amount,
            });
        }
        Ok(Money {
            amount: result,
            currency: self.currency,
        })
    }
}

impl Add for Money {
    type Output = Result<Money, DomainError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(DomainError::CurrencyMismatch {
                expected: self.currency,
                got: rhs.currency,
            });
        }
        Ok(Money {
            amount: self.amount + rhs.amount,
            currency: self.currency,
        })
    }
}

impl Sub for Money {
    type Output = Result<Money, DomainError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn rejects_negative_amount() {
        assert!(Money::new(dec!(-1), Currency::THB).is_err());
    }

    #[test]
    fn add_same_currency() {
        let a = Money::new(dec!(100), Currency::THB).unwrap();
        let b = Money::new(dec!(50), Currency::THB).unwrap();
        let result = (a + b).unwrap();
        assert_eq!(result.amount(), dec!(150));
    }

    #[test]
    fn add_different_currency_fails() {
        let a = Money::new(dec!(100), Currency::THB).unwrap();
        let b = Money::new(dec!(50), Currency::USD).unwrap();
        assert!((a + b).is_err());
    }

    #[test]
    fn sub_insufficient_funds() {
        let a = Money::new(dec!(10), Currency::THB).unwrap();
        let b = Money::new(dec!(50), Currency::THB).unwrap();
        assert!(a.checked_sub(b).is_err());
    }
}
