use std::io::{self, Write};
use std::fs::{self, File};
use std::env;

fn main() {
    println!("Welcome kluless!");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "write" => write_note(),
            "read" => read_notes(),
            _ => println!("Unknown command"),
        }
    } else {
        println!("No command provided");
    }
}

fn write_note() {
    println!("Enter your note: ");
    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read line");
    let mut file = File::create("notes.txt").expect("Unable to create file");
    file.write_all(note.as_bytes()).expect("Unable to write data");
}

fn read_notes() {
    let notes = fs::read_to_string("notes.txt").expect("Unable to read file");
    println!("Your notes:\n{}", notes);
}
