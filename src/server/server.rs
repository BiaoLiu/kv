use crate::service::{Service, ServiceInner};
use crate::storage::memory::MemTable;
async fn main() {
    let service: Service = ServiceInner::new(MemTable::new()).into();
}
