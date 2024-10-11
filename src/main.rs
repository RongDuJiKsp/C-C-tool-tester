mod application;
mod chicken;
mod hacker;
mod common;
mod init;

fn main() {
    init::__sync__init__();
    tokio::runtime::Runtime::new()
        .expect("Runtine Init Failed")
        .block_on(application::run_application());
}
