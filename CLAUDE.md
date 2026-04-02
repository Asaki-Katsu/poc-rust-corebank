# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                          # Build the project
cargo test                           # Run all unit + integration tests
cargo test --lib                     # Unit tests only
cargo test --test api_integration    # Integration tests only
cargo test <test_name>               # Run a single test by name
cargo run                            # Start the server on 0.0.0.0:3000
RUST_LOG=debug cargo run             # Start with debug-level tracing
cargo clippy                         # Lint
cargo fmt -- --check                 # Check formatting
```

## Architecture

This is a core-banking REST API built with **Axum** on **Tokio**, following **Clean Architecture** with four layers. Dependencies point inward only — domain has zero external dependencies.

### Layer Dependency Rule

```
Interface → Application → Domain
Infrastructure → Application (implements port traits)
```

### Layers

- **Domain** (`src/domain/`) — Pure business logic with no framework deps. Contains:
  - **Value Objects** (`value_objects/`): `Money` (decimal + currency, enforces non-negative, same-currency arithmetic), `AccountId`/`TransactionId` (newtype UUIDs preventing cross-entity misuse)
  - **Entities** (`entities/`): `Account` (balance mutations through domain methods enforcing invariants — active status, sufficient funds), `Transaction` (immutable financial record)
  - **Errors** (`errors.rs`): `DomainError` enum for all business rule violations

- **Application** (`src/application/`) — Use cases and port definitions:
  - **Ports** (`ports/`): `AccountRepository` and `TransactionRepository` traits using `impl Future` return types (no `#[async_trait]` needed — Rust 2024 edition)
  - **Use Cases** (`use_cases/`): Stateless async functions (`execute(repo, input)`) for create_account, deposit, withdraw, transfer, get_account, list_accounts, get_transactions

- **Infrastructure** (`src/infrastructure/`) — Port implementations:
  - `InMemoryAccountRepository` / `InMemoryTransactionRepository`: `Arc<RwLock<HashMap>>` for thread-safe concurrent access. Swap these for real DB implementations.

- **Interface** (`src/interface/http/`) — Axum HTTP layer:
  - `AppState<A, T>`: Generic over repository types — compile-time dependency injection
  - `routes.rs`: All endpoint definitions in one place
  - `error.rs`: `ApiError` maps `DomainError` variants to HTTP status codes (404, 422, 409, 400, 500)
  - `dto/`: Request/response types decoupled from domain entities

### Key Design Decisions

- **No `#[async_trait]`**: Repository traits use `impl Future` in trait methods (Rust 2024 edition RPITIT)
- **Monomorphic generics over `dyn` dispatch**: `AppState<A, T>` and handler functions are generic — zero-cost abstractions, no vtable overhead
- **`Money` arithmetic via `std::ops`**: `Add`/`Sub` impls return `Result`, enforcing currency match and sufficient funds at every arithmetic operation
- **`rust_decimal`**: All monetary math uses `Decimal` — no floating-point imprecision

### API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/accounts` | Create account |
| GET | `/accounts` | List all accounts |
| GET | `/accounts/{id}` | Get account by ID |
| POST | `/accounts/{id}/deposit` | Deposit to account |
| POST | `/accounts/{id}/withdraw` | Withdraw from account |
| POST | `/accounts/{id}/transfer` | Transfer between accounts |
| GET | `/accounts/{id}/transactions` | List account transactions |
