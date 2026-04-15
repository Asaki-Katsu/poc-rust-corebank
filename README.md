# Core Banking API

A REST API service for core banking operations built with Rust, Axum, and Tokio. Follows Clean Architecture with compile-time dependency injection and zero-cost abstractions.

## Features

- Account management (create, list, lookup)
- Deposits, withdrawals, and account-to-account transfers
- Transaction history per account
- Interest calculation endpoint (designed for cron-job triggers)
- All monetary math uses `rust_decimal` — no floating-point imprecision

## Architecture

Clean Architecture with four layers. Dependencies point inward only.

```text
Interface (HTTP) → Application (Use Cases) → Domain (Entities, Value Objects)
Infrastructure (Repos) → Application (implements port traits)
```

- **Domain** — Pure business logic. `Money` value object enforces non-negative balances and same-currency arithmetic. Newtype IDs prevent cross-entity misuse.
- **Application** — Stateless use-case functions and repository port traits. Uses Rust 2024 edition RPITIT — no `#[async_trait]` needed.
- **Infrastructure** — In-memory repository implementations (`Arc<RwLock<HashMap>>`). Swap these for real database adapters.
- **Interface** — Axum handlers, DTOs, and error mapping. `AppState<A, T>` is generic over repository types for compile-time DI.

## API

| Method | Path | Description |
| -------- | ------ | ------------- |
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

## Prerequisites

### Rust toolchain

Install Rust via [rustup](https://rustup.rs/) (the official installer):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This project uses the **2024 edition**, which requires Rust **nightly** or a recent stable release that supports it. After installing, make sure you have the latest toolchain:

```bash
rustup update
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### IDE setup (recommended)

Install [rust-analyzer](https://rust-analyzer.github.io/) for your IDE — it provides autocomplete, inline type hints, go-to-definition, and real-time error checking:

- **VS Code** — install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- **JetBrains (IntelliJ, CLion, RustRover)** — rust-analyzer is built-in or available as a plugin
- **Neovim / other LSP-compatible editors** — configure via your LSP client

### Tech stack

| Dependency | Purpose |
| --- | --- |
| [Axum](https://github.com/tokio-rs/axum) | HTTP framework |
| [Tokio](https://tokio.rs/) | Async runtime |
| [Serde](https://serde.rs/) | JSON serialization/deserialization |
| [rust_decimal](https://github.com/paupino/rust-decimal) | Precise decimal arithmetic for money |
| [uuid](https://github.com/uuid-rs/uuid) | Unique ID generation |
| [chrono](https://github.com/chronotope/chrono) | Date/time handling |
| [tracing](https://github.com/tokio-rs/tracing) | Structured logging |
| [tower-http](https://github.com/tower-rs/tower-http) | HTTP middleware (CORS, tracing) |

All dependencies are managed by Cargo — just run `cargo build` and they'll be fetched automatically.
