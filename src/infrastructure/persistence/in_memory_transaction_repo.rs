use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::ports::{TransactionRepository, TransactionRepositoryError};
use crate::domain::entities::Transaction;
use crate::domain::value_objects::AccountId;

/// Thread-safe in-memory transaction store.
#[derive(Debug, Clone, Default)]
pub struct InMemoryTransactionRepository {
    store: Arc<RwLock<HashMap<AccountId, Vec<Transaction>>>>,
}

impl InMemoryTransactionRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransactionRepository for InMemoryTransactionRepository {
    async fn save(&self, transaction: &Transaction) -> Result<(), TransactionRepositoryError> {
        let mut store = self.store.write().await;
        store
            .entry(transaction.account_id())
            .or_default()
            .push(transaction.clone());
        Ok(())
    }

    async fn find_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<Transaction>, TransactionRepositoryError> {
        let store = self.store.read().await;
        Ok(store.get(&account_id).cloned().unwrap_or_default())
    }
}
