use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use chrono::{Local, Timelike};

fn main() {
    let mut input_time = String::new();
    let mut input_task = String::new();

    print!("Enter the time for the task (in minutes): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_time).expect("Failed to read line");
    let minutes: u64 = input_time.trim().parse().expect("Please type a number!");

    print!("Enter the task: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_task).expect("Failed to read line");
    let task = input_task.trim();

    println!("Starting Pomodoro timer for '{}' for {} minutes...", task, minutes);

    let total_seconds = minutes * 60;
    for i in 0..total_seconds {
        clear_console();
        print_time_and_task(task, (total_seconds - i) / 60, (total_seconds - i) % 60);
        display_moving_symbol(i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("\nTime's up! Task '{}' completed!", task);
}

fn print_time_and_task(task: &str, minutes: u64, seconds: u64) {
    println!();
    println!("====================================");
    println!("  Task: {}  ", task);
    println!("====================================");
    println!();
    println!("        TIME LEFT        ");
    println!("==========================");
    println!("       {:02}:{:02}         ", minutes, seconds);
    println!("==========================");
}

fn display_moving_symbol(iteration: u64) {
    let symbols = ["|", "/", "-", "\\"];
    let symbol = symbols[(iteration % symbols.len() as u64) as usize];
    println!("         {}", symbol);
}

fn clear_console() {
    // This will clear the console in most Unix-based terminals and Windows Command Prompt
    print!("\x1B[2J\x1B[1;1H");
}
