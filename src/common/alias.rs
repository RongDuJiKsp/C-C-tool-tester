use crate::common::sync::Shared;
use std::future::Future;
use tokio::fs::File;
use tokio::task::JoinHandle;

pub type StdinHd = Shared<File>;
pub type StdoutHd = Shared<File>;
pub type StderrHd = Shared<File>;
pub fn go<T: Future>(f: T) -> JoinHandle<T::Output>
where
    T: Send + 'static,
    <T as Future>::Output: Send,
{
    tokio::spawn(f)
}
