# Core Banking API

A REST API for core banking operations built with Rust, Axum, and Tokio. Follows Clean Architecture with compile-time dependency injection and zero-cost abstractions.

## Features

- Account management (create, list, lookup)
- Deposits, withdrawals, and account-to-account transfers
- Transaction history per account
- Interest calculation endpoint (designed for cron-job triggers)
- All monetary math uses `rust_decimal` — no floating-point imprecision

## Architecture

Clean Architecture with four layers. Dependencies point inward only.

```
Interface (HTTP) → Application (Use Cases) → Domain (Entities, Value Objects)
Infrastructure (Repos) → Application (implements port traits)
```

- **Domain** — Pure business logic. `Money` value object enforces non-negative balances and same-currency arithmetic. Newtype IDs prevent cross-entity misuse.
- **Application** — Stateless use-case functions and repository port traits. Uses Rust 2024 edition RPITIT — no `#[async_trait]` needed.
- **Infrastructure** — In-memory repository implementations (`Arc<RwLock<HashMap>>`). Swap these for real database adapters.
- **Interface** — Axum handlers, DTOs, and error mapping. `AppState<A, T>` is generic over repository types for compile-time DI.

## API

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Health check |
| `POST` | `/accounts` | Create account |
| `GET` | `/accounts` | List all accounts |
| `GET` | `/accounts/{id}` | Get account by ID |
| `POST` | `/accounts/{id}/deposit` | Deposit |
| `POST` | `/accounts/{id}/withdraw` | Withdraw |
| `POST` | `/accounts/{id}/transfer` | Transfer between accounts |
| `GET` | `/accounts/{id}/transactions` | Transaction history |
| `POST` | `/interest/calculate` | Calculate interest |

## Getting Started

```bash
# Build
cargo build

# Run (listens on 0.0.0.0:3000)
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Lint & format check
cargo clippy && cargo fmt -- --check
```

## Requirements

- Rust (2024 edition)
