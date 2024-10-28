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
#[test]
fn test() {
    //test extract_value
    let input = "hello Rust AreYouOk";
    let template = "hello &[lang] AreYouOk";
    let placeholder = "&[lang]";

    let d = match common::strings::StringPkg::extract_value(input, template, placeholder) {
        Some(value) => format!("Extracted value: {}", value),
        None => "No match found".to_string(),
    };
    assert_eq!(d, "Extracted value: Rust".to_string());
}
