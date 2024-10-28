use std::fmt::Display;
use std::process::exit;

pub struct Log;
impl Log {
    pub fn result<T, E: Display>(r: Result<T, E>) {
        if let Err(e) = r {
            println!("{}", e);
        }
    }
    pub fn panic(t: &str) -> ! {
        println!("{}", t);
        exit(0)
    }
}
