use tokio::fs::File;
use crate::common::sync::Shared;

pub type StdinHd = Shared<File>;
pub type StdoutHd = Shared<File>;
pub type StderrHd = Shared<File>;
