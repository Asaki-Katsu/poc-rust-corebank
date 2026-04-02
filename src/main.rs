use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use core_banking::infrastructure::persistence::{
    InMemoryAccountRepository, InMemoryTransactionRepository,
};
use core_banking::interface::http::app_state::AppState;
use core_banking::interface::http::routes::build_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .json()
        .init();

    let state = AppState {
        account_repo: InMemoryAccountRepository::new(),
        transaction_repo: InMemoryTransactionRepository::new(),
    };

    let app = build_router(state).layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000";
    tracing::info!("listening on {addr}");
    let listener = TcpListener::bind(addr).await.expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}
