use crate::application::errors::ApplicationError;
use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::domain::entities::{Transaction, TransactionKind};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AccountId, Money};

pub struct WithdrawInput {
    pub account_id: AccountId,
    pub amount: Money,
    pub description: String,
}

/// Withdraws money from an account and records the transaction.
pub async fn execute(
    account_repo: &impl AccountRepository,
    tx_repo: &impl TransactionRepository,
    input: WithdrawInput,
) -> Result<Transaction, ApplicationError> {
    let mut account = account_repo
        .find_by_id(input.account_id)
        .await?
        .ok_or(DomainError::AccountNotFound(input.account_id))?;

    account.withdraw(input.amount)?;
    account_repo.save(&account).await?;

    let tx = Transaction::new(
        account.id(),
        None,
        TransactionKind::Withdrawal,
        input.amount,
        account.balance(),
        input.description,
    );
    tx_repo.save(&tx).await?;

    Ok(tx)
}
