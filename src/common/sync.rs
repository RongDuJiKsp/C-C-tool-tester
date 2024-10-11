use std::sync::Arc;
use tokio::sync::Mutex;

pub type Shared<T> = Arc<Mutex<T>>;
pub struct Ptr;
impl Ptr {
    pub fn share<T>(_data: T) -> Shared<T> {
        Arc::new(Mutex::new(_data))
    }
}
pub trait Context {
    fn cancel(&mut self);
}