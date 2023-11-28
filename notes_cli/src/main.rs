use std::fs::{self, OpenOptions};

use std::io::{self, Write};
use std::env;
use rand::seq::SliceRandom;

fn main() {
    let welcome_messages = [
        "Unleash your potential klu!",
        "Conquer your day klu!",
        "Be the hero of your own story klu!",
        "Let's make some epic notes klu!",
        "Welcome, warrior of productivity, klu!"
    ];

    let mut rng = rand::thread_rng();
    let welcome_message = welcome_messages.choose(&mut rng).unwrap();
    println!("{}", welcome_message);

    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, command] if command == "write" => {
            write_note();
            println!("Note written successfully.");
        },
        [_, command] if command == "read" => {
            read_notes();
        },
        [_, command, note_ids @ ..] if command == "delete" => {
            let ids: Vec<usize> = note_ids.iter().filter_map(|id| id.parse().ok()).collect();
            if ids.is_empty() {
                println!("Invalid or no note IDs provided");
            } else {
                delete_notes(&ids);
                println!("Note(s) deleted successfully.");
            }
        }
        _ => println!("Unknown or incomplete command"),
    }
}

fn write_note() {
    println!("Enter your note (press Enter on an empty line to submit):");
    let mut note = String::new();
    let mut line = String::new();

    while io::stdin().read_line(&mut line).is_ok() {
        if line.trim() == ":submit" || line.trim().is_empty() {
            break;
        }
        note.push_str(&line);
        line.clear();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Unable to open file");

    writeln!(file, "{}", note.trim_end()).expect("Unable to write data");
}


fn read_notes() {
    let notes = match fs::read_to_string("notes.txt") {
        Ok(contents) => contents,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };

    println!("Displaying notes:");
    for (index, note) in notes.lines().enumerate() {
        println!("{}: {}", index + 1, note);
    }
}

fn delete_notes(note_ids: &[usize]) {
    let contents = match fs::read_to_string("notes.txt") {
        Ok(contents) => contents,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };

    let mut notes: Vec<&str> = contents.lines().collect();
    let mut deleted = 0;  // Declare 'deleted' here

    for &note_id in note_ids.iter().rev() {
        if note_id == 0 || note_id > notes.len() {
            println!("Invalid note ID: {}", note_id);
            continue;
        }
        notes.remove(note_id - 1);
        deleted += 1;  // Increment 'deleted' for each successful deletion
    }

    let updated_contents = notes.join("\n");
    fs::write("notes.txt", updated_contents).expect("Error writing to file");

    println!("{} note(s) deleted successfully.", deleted);
}


