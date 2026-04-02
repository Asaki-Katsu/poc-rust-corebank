use crate::application::errors::ApplicationError;
use crate::application::ports::AccountRepository;
use crate::domain::entities::Account;
use crate::domain::value_objects::Currency;

pub struct CreateAccountInput {
    pub holder_name: String,
    pub currency: Currency,
}

/// Opens a new bank account with zero balance.
pub async fn execute(
    repo: &impl AccountRepository,
    input: CreateAccountInput,
) -> Result<Account, ApplicationError> {
    let account = Account::open(input.holder_name, input.currency)?;
    repo.save(&account).await?;
    Ok(account)
}
