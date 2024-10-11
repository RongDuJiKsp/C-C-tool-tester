mod application;

fn main() {
    tokio::runtime::Runtime::new()
        .expect("Runtine Init Failed")
        .block_on(application::run_application());

}
