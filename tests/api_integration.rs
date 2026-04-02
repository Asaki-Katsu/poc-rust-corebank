use axum::http::StatusCode;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use core_banking::infrastructure::persistence::{
    InMemoryAccountRepository, InMemoryTransactionRepository,
};
use core_banking::interface::http::app_state::AppState;
use core_banking::interface::http::routes::build_router;

fn test_app() -> axum::Router {
    let state = AppState {
        account_repo: InMemoryAccountRepository::new(),
        transaction_repo: InMemoryTransactionRepository::new(),
    };
    build_router(state)
}

fn json_request(method: &str, uri: &str, body: Value) -> axum::http::Request<String> {
    axum::http::Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .unwrap()
}

async fn response_json(response: axum::http::Response<axum::body::Body>) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn health_check() {
    let app = test_app();
    let req = axum::http::Request::builder()
        .uri("/health")
        .body(String::new())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_account_and_deposit() {
    let app = test_app();

    // Create account
    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Alice", "currency": "THB" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = response_json(resp).await;
    let account_id = body["id"].as_str().unwrap();
    assert_eq!(body["holder_name"], "Alice");
    assert_eq!(body["balance"], "0");

    // Deposit
    let req = json_request(
        "POST",
        &format!("/accounts/{account_id}/deposit"),
        json!({ "amount": "1000.50", "currency": "THB" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = response_json(resp).await;
    assert_eq!(body["balance_after"], "1000.50");
    assert_eq!(body["kind"], "Deposit");

    // Get account — verify balance
    let req = axum::http::Request::builder()
        .uri(&format!("/accounts/{account_id}"))
        .body(String::new())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let body = response_json(resp).await;
    assert_eq!(body["balance"], "1000.50");
}

#[tokio::test]
async fn transfer_between_accounts() {
    let app = test_app();

    // Create two accounts
    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Alice", "currency": "USD" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let alice_id = response_json(resp).await["id"].as_str().unwrap().to_string();

    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Bob", "currency": "USD" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let bob_id = response_json(resp).await["id"].as_str().unwrap().to_string();

    // Fund Alice
    let req = json_request(
        "POST",
        &format!("/accounts/{alice_id}/deposit"),
        json!({ "amount": "500", "currency": "USD" }),
    );
    app.clone().oneshot(req).await.unwrap();

    // Transfer Alice -> Bob
    let req = json_request(
        "POST",
        &format!("/accounts/{alice_id}/transfer"),
        json!({ "to_account_id": bob_id, "amount": "200", "currency": "USD" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify balances
    let req = axum::http::Request::builder()
        .uri(&format!("/accounts/{alice_id}"))
        .body(String::new())
        .unwrap();
    let alice = response_json(app.clone().oneshot(req).await.unwrap()).await;
    assert_eq!(alice["balance"], "300");

    let req = axum::http::Request::builder()
        .uri(&format!("/accounts/{bob_id}"))
        .body(String::new())
        .unwrap();
    let bob = response_json(app.oneshot(req).await.unwrap()).await;
    assert_eq!(bob["balance"], "200");
}

#[tokio::test]
async fn calculate_interest_on_accounts() {
    let app = test_app();

    // Create two accounts
    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Alice", "currency": "THB" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let alice_id = response_json(resp).await["id"].as_str().unwrap().to_string();

    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Bob", "currency": "THB" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let bob_id = response_json(resp).await["id"].as_str().unwrap().to_string();

    // Fund Alice with 10,000 THB, Bob with 0 (should be skipped)
    let req = json_request(
        "POST",
        &format!("/accounts/{alice_id}/deposit"),
        json!({ "amount": "10000", "currency": "THB" }),
    );
    app.clone().oneshot(req).await.unwrap();

    // Calculate monthly interest at 2.5% annual rate
    let req = json_request(
        "POST",
        "/interest/calculate",
        json!({ "annual_rate": "0.025", "periods_per_year": "12" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = response_json(resp).await;
    assert_eq!(body["accounts_processed"], 1);
    assert_eq!(body["accounts_skipped"], 1); // Bob has zero balance
    assert_eq!(body["transactions"].as_array().unwrap().len(), 1);

    let tx = &body["transactions"][0];
    assert_eq!(tx["kind"], "Interest");
    // 10000 * 0.025 / 12 ≈ 20.833333...
    assert!(tx["amount"].as_str().unwrap().starts_with("20.8333"));

    // Verify Alice's balance increased
    let req = axum::http::Request::builder()
        .uri(&format!("/accounts/{alice_id}"))
        .body(String::new())
        .unwrap();
    let alice = response_json(app.clone().oneshot(req).await.unwrap()).await;
    let balance: f64 = alice["balance"].as_str().unwrap().parse().unwrap();
    assert!(balance > 10000.0);

    // Verify Bob unchanged
    let req = axum::http::Request::builder()
        .uri(&format!("/accounts/{bob_id}"))
        .body(String::new())
        .unwrap();
    let bob = response_json(app.oneshot(req).await.unwrap()).await;
    assert_eq!(bob["balance"], "0");
}

#[tokio::test]
async fn insufficient_funds_returns_422() {
    let app = test_app();

    let req = json_request(
        "POST",
        "/accounts",
        json!({ "holder_name": "Charlie", "currency": "THB" }),
    );
    let resp = app.clone().oneshot(req).await.unwrap();
    let id = response_json(resp).await["id"].as_str().unwrap().to_string();

    let req = json_request(
        "POST",
        &format!("/accounts/{id}/withdraw"),
        json!({ "amount": "100", "currency": "THB" }),
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
