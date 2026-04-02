use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::application::use_cases::{create_account, get_account, list_accounts};
use crate::domain::value_objects::AccountId;
use crate::interface::http::app_state::AppState;
use crate::interface::http::dto::{AccountResponse, CreateAccountRequest};
use crate::interface::http::error::ApiError;

pub async fn create<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Json(body): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let input = create_account::CreateAccountInput {
        holder_name: body.holder_name,
        currency: body.currency,
    };
    let account = create_account::execute(&state.account_repo, input).await?;
    Ok(Json(account.into()))
}

pub async fn get<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountResponse>, ApiError> {
    let account = get_account::execute(&state.account_repo, AccountId::from(id)).await?;
    Ok(Json(account.into()))
}

pub async fn list<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
) -> Result<Json<Vec<AccountResponse>>, ApiError> {
    let accounts = list_accounts::execute(&state.account_repo).await?;
    Ok(Json(accounts.into_iter().map(Into::into).collect()))
}
