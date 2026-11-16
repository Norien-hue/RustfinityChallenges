// Define a struct named `Logger`
// Implement an associated function `log_message`
// That accepts a `&str` and prints the output.
pub struct Logger;

impl Logger {
    pub fn log_message(msg: &str) -> () {
        println!("{}", msg);
    }
}

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}
