use crate::domain::entities::Account;
use crate::domain::value_objects::AccountId;

/// Port for account persistence — implemented by infrastructure layer.
/// Uses `Send + Sync` bounds so implementations can be shared across async tasks.
pub trait AccountRepository: Send + Sync {
    fn save(
        &self,
        account: &Account,
    ) -> impl Future<Output = Result<(), AccountRepositoryError>> + Send;

    fn find_by_id(
        &self,
        id: AccountId,
    ) -> impl Future<Output = Result<Option<Account>, AccountRepositoryError>> + Send;

    fn find_all(
        &self,
    ) -> impl Future<Output = Result<Vec<Account>, AccountRepositoryError>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum AccountRepositoryError {
    #[error("persistence error: {0}")]
    Internal(String),
}
