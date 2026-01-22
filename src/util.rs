use colored::*;

pub fn log_info(msg: &str) { println!("{} {}", "ℹ".blue(), msg); }
pub fn log_success(msg: &str) { println!("{} {}", "✔".green(), msg); }
pub fn log_error(msg: &str) { eprintln!("{} {}", "✖".red(), msg); }