use crate::application::errors::ApplicationError;
use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::domain::entities::{Transaction, TransactionKind};
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{AccountId, Money};

pub struct TransferInput {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: Money,
    pub description: String,
}

/// Transfers money between two accounts atomically (in-process).
/// Returns the pair of transactions (debit, credit).
pub async fn execute(
    account_repo: &impl AccountRepository,
    tx_repo: &impl TransactionRepository,
    input: TransferInput,
) -> Result<(Transaction, Transaction), ApplicationError> {
    if input.from_account_id == input.to_account_id {
        return Err(DomainError::SelfTransfer.into());
    }

    let mut from = account_repo
        .find_by_id(input.from_account_id)
        .await?
        .ok_or(DomainError::AccountNotFound(input.from_account_id))?;

    let mut to = account_repo
        .find_by_id(input.to_account_id)
        .await?
        .ok_or(DomainError::AccountNotFound(input.to_account_id))?;

    from.withdraw(input.amount)?;
    to.deposit(input.amount)?;

    account_repo.save(&from).await?;
    account_repo.save(&to).await?;

    let debit_tx = Transaction::new(
        from.id(),
        Some(to.id()),
        TransactionKind::TransferOut,
        input.amount,
        from.balance(),
        input.description.clone(),
    );

    let credit_tx = Transaction::new(
        to.id(),
        Some(from.id()),
        TransactionKind::TransferIn,
        input.amount,
        to.balance(),
        input.description,
    );

    tx_repo.save(&debit_tx).await?;
    tx_repo.save(&credit_tx).await?;

    Ok((debit_tx, credit_tx))
}
