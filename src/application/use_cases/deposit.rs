use crate::application::errors::ApplicationError;
use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::domain::entities::{Transaction, TransactionKind};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AccountId, Money};

pub struct DepositInput {
    pub account_id: AccountId,
    pub amount: Money,
    pub description: String,
}

/// Deposits money into an account and records the transaction.
pub async fn execute(
    account_repo: &impl AccountRepository,
    tx_repo: &impl TransactionRepository,
    input: DepositInput,
) -> Result<Transaction, ApplicationError> {
    let mut account = account_repo
        .find_by_id(input.account_id)
        .await?
        .ok_or(DomainError::AccountNotFound(input.account_id))?;

    account.deposit(input.amount)?;
    account_repo.save(&account).await?;

    let tx = Transaction::new(
        account.id(),
        None,
        TransactionKind::Deposit,
        input.amount,
        account.balance(),
        input.description,
    );
    tx_repo.save(&tx).await?;

    Ok(tx)
}
