use axum::extract::State;
use axum::Json;

use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::application::use_cases::calculate_interest;
use crate::interface::http::app_state::AppState;
use crate::interface::http::dto::{CalculateInterestRequest, CalculateInterestResponse};
use crate::interface::http::error::ApiError;

pub async fn calculate<A: AccountRepository, T: TransactionRepository>(
    State(state): State<AppState<A, T>>,
    Json(body): Json<CalculateInterestRequest>,
) -> Result<Json<CalculateInterestResponse>, ApiError> {
    let input = calculate_interest::CalculateInterestInput {
        annual_rate: body.annual_rate,
        periods_per_year: body.periods_per_year,
    };
    let result =
        calculate_interest::execute(&state.account_repo, &state.transaction_repo, input).await?;
    Ok(Json(result.into()))
}
