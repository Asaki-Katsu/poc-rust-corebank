use crate::application::errors::ApplicationError;
use crate::application::ports::AccountRepository;
use crate::domain::entities::Account;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::AccountId;

/// Retrieves a single account by ID.
pub async fn execute(
    repo: &impl AccountRepository,
    id: AccountId,
) -> Result<Account, ApplicationError> {
    repo.find_by_id(id)
        .await?
        .ok_or_else(|| DomainError::AccountNotFound(id).into())
}
