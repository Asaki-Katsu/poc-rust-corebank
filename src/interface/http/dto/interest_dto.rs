use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::TransactionResponse;
use crate::application::use_cases::calculate_interest::InterestResult;

#[derive(Debug, Deserialize)]
pub struct CalculateInterestRequest {
    /// Annual interest rate as a decimal (e.g. 0.025 for 2.5%).
    pub annual_rate: Decimal,
    /// Number of compounding periods per year (e.g. 12 for monthly, 365 for daily).
    pub periods_per_year: Decimal,
}

#[derive(Debug, Serialize)]
pub struct CalculateInterestResponse {
    pub accounts_processed: usize,
    pub accounts_skipped: usize,
    pub transactions: Vec<TransactionResponse>,
}

impl From<InterestResult> for CalculateInterestResponse {
    fn from(result: InterestResult) -> Self {
        Self {
            accounts_processed: result.accounts_processed,
            accounts_skipped: result.accounts_skipped,
            transactions: result.transactions.into_iter().map(Into::into).collect(),
        }
    }
}
