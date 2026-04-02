use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::application::use_cases::{deposit, get_transactions, transfer, withdraw};
use crate::domain::value_objects::{AccountId, Money};
use crate::interface::http::app_state::AppState;
use crate::interface::http::dto::{
    DepositRequest, TransactionResponse, TransferRequest, WithdrawRequest,
};
use crate::interface::http::error::ApiError;

pub async fn do_deposit<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Path(account_id): Path<Uuid>,
    Json(body): Json<DepositRequest>,
) -> Result<Json<TransactionResponse>, ApiError> {
    let amount = Money::new(body.amount, body.currency)
        .map_err(|e| ApiError(e.into()))?;

    let input = deposit::DepositInput {
        account_id: AccountId::from(account_id),
        amount,
        description: body.description.unwrap_or_else(|| "Deposit".into()),
    };
    let tx = deposit::execute(&state.account_repo, &state.transaction_repo, input).await?;
    Ok(Json(tx.into()))
}

pub async fn do_withdraw<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Path(account_id): Path<Uuid>,
    Json(body): Json<WithdrawRequest>,
) -> Result<Json<TransactionResponse>, ApiError> {
    let amount = Money::new(body.amount, body.currency)
        .map_err(|e| ApiError(e.into()))?;

    let input = withdraw::WithdrawInput {
        account_id: AccountId::from(account_id),
        amount,
        description: body.description.unwrap_or_else(|| "Withdrawal".into()),
    };
    let tx = withdraw::execute(&state.account_repo, &state.transaction_repo, input).await?;
    Ok(Json(tx.into()))
}

pub async fn do_transfer<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Path(from_account_id): Path<Uuid>,
    Json(body): Json<TransferRequest>,
) -> Result<Json<(TransactionResponse, TransactionResponse)>, ApiError> {
    let amount = Money::new(body.amount, body.currency)
        .map_err(|e| ApiError(e.into()))?;

    let input = transfer::TransferInput {
        from_account_id: AccountId::from(from_account_id),
        to_account_id: AccountId::from(body.to_account_id),
        amount,
        description: body.description.unwrap_or_else(|| "Transfer".into()),
    };
    let (debit, credit) =
        transfer::execute(&state.account_repo, &state.transaction_repo, input).await?;
    Ok(Json((debit.into(), credit.into())))
}

pub async fn list_for_account<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<TransactionResponse>>, ApiError> {
    let txs =
        get_transactions::execute(&state.transaction_repo, AccountId::from(account_id)).await?;
    Ok(Json(txs.into_iter().map(Into::into).collect()))
}
