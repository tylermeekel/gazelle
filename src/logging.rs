use colored::Colorize;
use chrono::prelude::*;

fn log(message: String) {
    let time = Utc::now();
    let formatted_time = time.format("%Y-%m-%d | %H:%M:%S");

    println!("({}) {}", formatted_time, message)
}

pub fn log_core(message: &str) {
    log(format!("[{}]: {}", "CORE".red(), message));
}

pub fn log_client() {}