use chrono::prelude::*;

fn main() {
    let local_time = Local::now();

    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Current date and time: {}", formatted_time);
}
