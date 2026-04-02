use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::application::errors::ApplicationError;
use crate::domain::errors::DomainError;

/// Maps application errors to HTTP responses with appropriate status codes.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            ApplicationError::Domain(e) => {
                let status = match e {
                    DomainError::AccountNotFound(_) => StatusCode::NOT_FOUND,
                    DomainError::InsufficientFunds { .. } => StatusCode::UNPROCESSABLE_ENTITY,
                    DomainError::CurrencyMismatch { .. } => StatusCode::UNPROCESSABLE_ENTITY,
                    DomainError::AccountFrozen => StatusCode::CONFLICT,
                    DomainError::SelfTransfer => StatusCode::UNPROCESSABLE_ENTITY,
                    _ => StatusCode::BAD_REQUEST,
                };
                (status, e.to_string())
            }
            ApplicationError::AccountRepo(e) => {
                tracing::error!("account repository error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
            ApplicationError::TransactionRepo(e) => {
                tracing::error!("transaction repository error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };

        let body = json!({ "error": message });
        (status, axum::Json(body)).into_response()
    }
}

/// Newtype wrapper so we can implement `IntoResponse` for foreign type.
pub struct ApiError(pub ApplicationError);

impl From<ApplicationError> for ApiError {
    fn from(e: ApplicationError) -> Self {
        Self(e)
    }
}
