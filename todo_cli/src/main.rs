extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

enum Command {
    Add(String),
    Complete(usize),
    Delete(usize),
    List,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.as_slice() {
        // Note: We're using `args[2..].join(" ")` to correctly get the description as one string.
        [_, cmd, desc, ..] if cmd == "add" => Command::Add(desc.to_string()),

        [_, cmd, id] if cmd == "complete" => Command::Complete(id.parse().unwrap()),
        [_, cmd, id] if cmd == "delete" => Command::Delete(id.parse().unwrap()),
        [_, cmd] if cmd == "list" => Command::List,
        _ => {
            eprintln!("Invalid command!");
            return;
        }
    };

    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    match command {
        Command::Add(description) => {
            tasks.push(Task { 
                id: tasks.len() + 1, 
                description, 
                completed: false 
            });
        },
        Command::Complete(id) => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.completed = true;
            }
        },
        Command::Delete(id) => {
            tasks.retain(|task| task.id != id);
        },
        Command::List => {
            for task in &tasks {
                println!("{:?}", task);
            }
        }
    }

    save_tasks(&tasks).expect("Failed to save tasks");
}

fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    let data = std::fs::read_to_string("tasks.json")?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(tasks)?;
    std::fs::write("tasks.json", data)?;
    Ok(())
}
