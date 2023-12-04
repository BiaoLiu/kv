use std::sync::Arc;
use crate::pb::abi::CommandResponse;
use crate::storage::Storage;

pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}


pub struct Service<Store> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }
}

impl<Store: Storage> ServiceInner<Store> {
    fn new(store: Store) -> Self {
        Self {
            store,
        }
    }
}
