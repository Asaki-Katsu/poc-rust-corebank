use axum::routing::{get, post};
use axum::Router;

use crate::application::ports::{AccountRepository, TransactionRepository};
use crate::interface::http::app_state::AppState;
use crate::interface::http::handlers::{account_handlers, interest_handlers, transaction_handlers};

pub fn build_router<A, T>(state: AppState<A, T>) -> Router
where
    A: AccountRepository + Clone + 'static,
    T: TransactionRepository + Clone + 'static,
{
    Router::new()
        // Health check
        .route("/health", get(|| async { "ok" }))
        // Account endpoints
        .route("/accounts", post(account_handlers::create::<A, T>))
        .route("/accounts", get(account_handlers::list::<A, T>))
        .route("/accounts/{id}", get(account_handlers::get::<A, T>))
        // Transaction endpoints
        .route(
            "/accounts/{id}/deposit",
            post(transaction_handlers::do_deposit::<A, T>),
        )
        .route(
            "/accounts/{id}/withdraw",
            post(transaction_handlers::do_withdraw::<A, T>),
        )
        .route(
            "/accounts/{id}/transfer",
            post(transaction_handlers::do_transfer::<A, T>),
        )
        .route(
            "/accounts/{id}/transactions",
            get(transaction_handlers::list_for_account::<A, T>),
        )
        // Interest endpoints (cron-job triggered)
        .route(
            "/interest/calculate",
            post(interest_handlers::calculate::<A, T>),
        )
        .with_state(state)
}
