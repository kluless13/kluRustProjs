extern crate serde_json;
use rand::seq::SliceRandom; // For random selection from a slice
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    time_estimate: String,
    designation: String,
}

fn print_welcome_message(designation: Option<&String>) {
    let messages = [
        "Let's have a productive day!",
        "Let's tackle today's challenges!",
        "Time to turn goals into achievements!",
        "Ready to check off some tasks?",
        "Kill it.",
        "You're creating change.",
    ];

    let greeting = match designation {
        Some(d) => format!("Welcome {} of Oranj, ", d),
        None => "Welcome team Oranj, ".to_string(),
    };

    let mut rng = rand::thread_rng();
    let message = messages.choose(&mut rng).unwrap();
    println!("{}{}", greeting, message);
}

fn print_tasks(tasks: &[Task], designation_filter: Option<&String>) {
    println!("{:<5} | {:<20} | {:<15} | {:<10} | {:<12}", "ID", "Description", "Time Estimate", "Completed", "Designation");
    println!("{:-<5}-+-{:-<20}-+-{:-<15}-+-{:-<10}-+-{:-<12}", "", "", "", "", ""); // Line below headings
    for task in tasks.iter().filter(|task| match designation_filter {
        Some(filter) => &task.designation == filter,
        None => true,
    }) {
        println!("{:<5} | {:<20} | {:<15} | {:<10} | {:<12}", task.id, task.description, task.time_estimate, if task.completed { "Yes" } else { "No" }, task.designation);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, cmd] if cmd == "list" => {
            let tasks = load_tasks().unwrap_or_else(|_| Vec::new());
            print_welcome_message(None);
            print_tasks(&tasks, None);
        },
        [_, cmd, designation] if cmd == "list" => {
            let tasks = load_tasks().unwrap_or_else(|_| Vec::new());
            print_welcome_message(Some(designation));
            print_tasks(&tasks, Some(designation));
        },
        [_, cmd, description, time_estimate, designation] if cmd == "add" => {
            let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());
            let task = Task {
                id: tasks.len() + 1,
                description: description.to_string(),
                completed: false,
                time_estimate: time_estimate.to_string(),
                designation: designation.to_string(),
            };
            tasks.push(task);
            save_tasks(&tasks).expect("Failed to save tasks.");
            println!("Task added: {}", description);
        },
        [_, cmd, id_str] if cmd == "complete" || cmd == "delete" => {
            let id: usize = id_str.parse().expect("ID should be a number");
            let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());
            if cmd == "complete" {
                if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                    task.completed = true;
                    println!("Task {} marked as completed.", id);
                }
            } else {
                tasks.retain(|task| task.id != id);
                println!("Task {} deleted.", id);
            }
            save_tasks(&tasks).expect("Failed to save tasks.");
            save_tasks_to_markdown(&tasks).expect("Failed to save tasks to Markdown.");
        },
        _ => eprintln!("Usage: cargo run -- [list | list <designation> | add <description> <time_estimate> <designation> | complete <id> | delete <id>]"),
    }
}

fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    let path = "tasks.json";
    match fs::read_to_string(path) {
        Ok(data) => serde_json::from_str(&data).or_else(|_| Ok(Vec::new())),
        Err(_) => Ok(Vec::new()), // Return an empty vector if the file doesn't exist
    }
}

fn save_tasks(tasks: &[Task]) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(tasks)?;
    fs::write("tasks.json", data)
}

fn save_tasks_to_markdown(tasks: &[Task]) -> Result<(), std::io::Error> {
    let mut content = String::new();
    content.push_str("| ID | Description | Time Estimate | Completed |\n");
    content.push_str("|----|-------------|---------------|-----------|\n");
    for task in tasks {
        let completed_text = if task.completed { "Yes" } else { "No" };
        content.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            task.id, task.description, task.time_estimate, completed_text
        ));
    }
    std::fs::write("tasks.md", content)?;
    Ok(())
}
