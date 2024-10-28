mod application;
mod chicken;
mod common;
mod hacker;
mod init;

fn main() {
    init::__sync_init();
    tokio::runtime::Runtime::new()
        .expect("Runtine Init Failed")
        .block_on(application::run_application());
}
