use crate::application::errors::ApplicationError;
use crate::application::ports::AccountRepository;
use crate::domain::entities::Account;

/// Lists all accounts.
pub async fn execute(repo: &impl AccountRepository) -> Result<Vec<Account>, ApplicationError> {
    Ok(repo.find_all().await?)
}
