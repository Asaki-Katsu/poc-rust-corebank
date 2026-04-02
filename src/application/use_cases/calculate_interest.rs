use rust_decimal::Decimal;

use crate::application::errors::ApplicationError;
use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::domain::entities::{AccountStatus, Transaction, TransactionKind};
use crate::domain::value_objects::Money;

pub struct CalculateInterestInput {
    /// Annual interest rate as a decimal (e.g. 0.025 for 2.5%).
    pub annual_rate: Decimal,
    /// Number of compounding periods per year (e.g. 12 for monthly, 365 for daily).
    pub periods_per_year: Decimal,
}

pub struct InterestResult {
    pub transactions: Vec<Transaction>,
    pub accounts_processed: usize,
    pub accounts_skipped: usize,
}

/// Calculates and applies interest to all active accounts.
/// Designed to be called by a cron job on a periodic schedule.
pub async fn execute(
    account_repo: &impl AccountRepository,
    tx_repo: &impl TransactionRepository,
    input: CalculateInterestInput,
) -> Result<InterestResult, ApplicationError> {
    let accounts = account_repo.find_all().await?;
    let periodic_rate = input.annual_rate / input.periods_per_year;

    let mut transactions = Vec::new();
    let mut accounts_skipped = 0usize;

    for account in &accounts {
        if account.status() != AccountStatus::Active || account.balance().is_zero() {
            accounts_skipped += 1;
            continue;
        }

        let interest_amount = account.balance().amount() * periodic_rate;
        let interest_money = Money::new(interest_amount, account.currency())?;

        if interest_money.is_zero() {
            accounts_skipped += 1;
            continue;
        }

        let mut account = account.clone();
        account.deposit(interest_money)?;
        account_repo.save(&account).await?;

        let tx = Transaction::new(
            account.id(),
            None,
            TransactionKind::Interest,
            interest_money,
            account.balance(),
            format!(
                "Interest accrual: {:.4}% annual rate",
                input.annual_rate * Decimal::ONE_HUNDRED
            ),
        );
        tx_repo.save(&tx).await?;
        transactions.push(tx);
    }

    let accounts_processed = transactions.len();
    Ok(InterestResult {
        transactions,
        accounts_processed,
        accounts_skipped,
    })
}
