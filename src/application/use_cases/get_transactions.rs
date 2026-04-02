use crate::application::errors::ApplicationError;
use crate::application::ports::TransactionRepository;
use crate::domain::entities::Transaction;
use crate::domain::value_objects::AccountId;

/// Lists all transactions for a given account.
pub async fn execute(
    tx_repo: &impl TransactionRepository,
    account_id: AccountId,
) -> Result<Vec<Transaction>, ApplicationError> {
    Ok(tx_repo.find_by_account_id(account_id).await?)
}
