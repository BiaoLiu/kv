mod command_service;

use crate::error::KvError;
use crate::pb::abi::command_request::RequestData;
use crate::pb::abi::{CommandRequest, CommandResponse};
use crate::storage::Storage;
use std::sync::Arc;
use tracing::debug;
use crate::storage::memory::MemTable;

pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> ServiceInner<Store> {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<Store: Storage> From<ServiceInner<Store>> for Service<Store> {
    fn from(inner: ServiceInner<Store>) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request:{:?}", cmd);
        match cmd.request_data {
            Some(RequestData::Hget(param)) => param.execute(&self.inner.store),
            Some(RequestData::Hgetall(param)) => param.execute(&self.inner.store),
            Some(RequestData::Hset(param)) => param.execute(&self.inner.store),
            None => KvError::InvalidCommand("Request has no data".into()).into(),
            _ => KvError::Internal("Not implemented".into()).into(),
        }
    }
}
