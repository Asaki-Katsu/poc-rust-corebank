use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::ports::{AccountRepository, AccountRepositoryError};
use crate::domain::entities::Account;
use crate::domain::value_objects::AccountId;

/// Thread-safe in-memory account store.
/// Uses `Arc<RwLock<_>>` for concurrent read access with exclusive writes.
#[derive(Debug, Clone, Default)]
pub struct InMemoryAccountRepository {
    store: Arc<RwLock<HashMap<AccountId, Account>>>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AccountRepository for InMemoryAccountRepository {
    async fn save(&self, account: &Account) -> Result<(), AccountRepositoryError> {
        let mut store = self.store.write().await;
        store.insert(account.id(), account.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: AccountId,
    ) -> Result<Option<Account>, AccountRepositoryError> {
        let store = self.store.read().await;
        Ok(store.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Account>, AccountRepositoryError> {
        let store = self.store.read().await;
        Ok(store.values().cloned().collect())
    }
}
