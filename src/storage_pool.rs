use napi::{Env, JsObject, Property, Result as NapiResult};
use virt;

#[napi]
pub struct StoragePool {
    storage_pool: virt::storage_pool::StoragePool
}

impl StoragePool {
    pub fn get(&self) -> &virt::storage_pool::StoragePool {
        &self.storage_pool
    }

    pub fn from_storage_pool(storage_pool: virt::storage_pool::StoragePool) -> Self {
        Self { storage_pool }
    }
}
