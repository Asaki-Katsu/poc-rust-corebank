use crate::domain::entities::Transaction;
use crate::domain::value_objects::AccountId;

/// Port for transaction persistence.
pub trait TransactionRepository: Send + Sync {
    fn save(
        &self,
        transaction: &Transaction,
    ) -> impl Future<Output = Result<(), TransactionRepositoryError>> + Send;

    fn find_by_account_id(
        &self,
        account_id: AccountId,
    ) -> impl Future<Output = Result<Vec<Transaction>, TransactionRepositoryError>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionRepositoryError {
    #[error("persistence error: {0}")]
    Internal(String),
}
