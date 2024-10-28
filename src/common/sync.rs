use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub type Shared<T> = Arc<Mutex<T>>;
pub type RWShared<T> = Arc<RwLock<T>>;
pub struct Ptr;
impl Ptr {
    pub fn share<T>(_data: T) -> Shared<T> {
        Arc::new(Mutex::new(_data))
    }
    pub fn rw_share<T>(t: T) -> RWShared<T> {
        Arc::new(RwLock::new(t))
    }
}
pub trait Context {
    fn cancel(&mut self);
}
