extern crate serde_json;

use rand::seq::SliceRandom; // For random selection from a slice

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    time_estimate: String,
}

#[derive(Debug)]
enum Command {
    Add(String, String),
    Complete(usize),
    Delete(usize),
    List,
}

fn print_welcome_message() {
    let messages = [
        "Welcome kluless, let's have a productive day!",
        "Welcome kluless, let's tackle today's challenges!",
        "Welcome kluless, time to turn goals into achievements!",
        "Welcome kluless, ready to check off some tasks?",
        "Welcome kluless, kill it.",
        "Welcome kluless, you're creating change.",
    ];

    let mut rng = rand::thread_rng();
    let message = messages.choose(&mut rng).unwrap();
    println!("{}", message);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.as_slice() {
        [_, cmd, desc, time_estimate, ..] if cmd == "add" => {
            Command::Add(desc.to_string(), time_estimate.to_string())
        }
        [_, cmd, id] if cmd == "complete" => Command::Complete(id.parse().unwrap()),
        [_, cmd, id] if cmd == "delete" => Command::Delete(id.parse().unwrap()),
        [_, cmd] if cmd == "list" => Command::List,
        _ => {
            eprintln!("Usage: add <description> <time_estimate> | complete <id> | delete <id> | list");
            return;
        }
    };

    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    match command {
    Command::Add(description, time_estimate) => {
        let task = Task {
            id: tasks.len() + 1,
            description: description.clone(), // Clone the description here
            completed: false,
            time_estimate,
        };
        tasks.push(task);
        println!("Task added: {}", description);       
    },
        Command::Complete(id) => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.completed = true;
                println!("Task {} marked as completed.", id);
            }
        },
        Command::Delete(id) => {
            tasks.retain(|task| task.id != id);
            println!("Task {} deleted.", id);
        },
        Command::List => {
            println!(); 
            println!();
            print_welcome_message(); 
            println!();
            println!();
            let max_id_width = tasks.iter().map(|task| task.id.to_string().len()).max().unwrap_or(2);
            let max_description_width = tasks.iter().map(|task| task.description.len()).max().unwrap_or(11);
            let max_time_estimate_width = tasks.iter().map(|task| task.time_estimate.len()).max().unwrap_or(13);
            let completed_width = "Completed".len();

            println!("{:<id_width$} | {:<description_width$} | {:<time_estimate_width$} | {:completed_width$}",
                 "ID", "Description", "Time Estimate", "Completed",
                id_width=max_id_width +1, description_width=max_description_width +1,
                time_estimate_width=max_time_estimate_width +1, completed_width=completed_width +1);
    
            println!("{:-<id_width$}-+-{:-<description_width$}-+-{:-<time_estimate_width$}-+-{:-<completed_width$}",
                "", "", "", "", 
                id_width=max_id_width +1, description_width=max_description_width +1,
                time_estimate_width=max_time_estimate_width +1, completed_width=completed_width +1);

        for task in &tasks {
            let completed_text = if task.completed { "Yes" } else { "No" };
            println!("{:<id_width$} | {:<description_width$} | {:<time_estimate_width$} | {:<completed_width$}",
                 task.id, task.description, task.time_estimate, completed_text,
                 id_width=max_id_width +1, description_width=max_description_width +1,
                 time_estimate_width=max_time_estimate_width +1, completed_width=completed_width +1);
            }
        },

    }

    save_tasks(&tasks).expect("Failed to save tasks");
}

fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    let data = std::fs::read_to_string("tasks.json")?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(tasks)?;
    std::fs::write("tasks.json", data)?;
    Ok(())
}

