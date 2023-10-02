use chrono::{NaiveDate, Utc, Datelike};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use serde_json;

// Define UserData Struct
#[derive(Serialize, Deserialize)]
struct UserData {
    responses: HashMap<i64, HashMap<i64, String>>, // Store daily responses
    week_calculated: HashMap<i64, bool>, // Added missing field
}

fn main() {
    let dob = get_user_dob();
    let user_responses = calculate_and_display_weeks(&dob);
    let user_data = UserData { 
        responses: user_responses, 
        week_calculated: HashMap::new() // Initialized the missing field
    };
    store_user_data(&user_data);
}

fn get_user_dob() -> String {
    println!("Enter your date of birth (YYYY-MM-DD):");
    let mut dob = String::new();
    io::stdin().read_line(&mut dob).expect("Failed to read line");
    dob.trim().to_string() // Return a trimmed String
}

fn calculate_and_display_weeks(dob: &str) -> HashMap<i64, HashMap<i64, String>> {
    let dob = NaiveDate::parse_from_str(dob, "%Y-%m-%d").expect("Invalid date format");
    let now = Utc::now().naive_local().date();


    let duration = now.signed_duration_since(dob);
    let weeks_passed = duration.num_weeks();
    
    let mut user_responses = HashMap::new();
    let mut diagram = String::new();

    for _ in 1..weeks_passed {
        diagram.push_str("\x1b[37m●\x1b[0m ");
    }

    let current_weekday = now.weekday().number_from_sunday();
    
    let mut week_responses = HashMap::new();
    let mut total = 0;
    let mut count = 0;
    for day in 1..=current_weekday {
        println!("Current Week (Week {}) Day {}: Was it a good day or a bad day? (Enter 'good' or 'bad', 'skip' to leave neutral)", weeks_passed, day);
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");
        let response = response.trim().to_lowercase();
        
        match response.as_str() {
            "good" => {
                week_responses.insert(day.into(), response); // Converted day to i64
                total += 1;
                count += 1;
            }
            "bad" => {
                week_responses.insert(day.into(), response); // Converted day to i64
                total -= 1;
                count += 1;
            }
            "skip" => {}
            _ => {
                println!("Invalid response. Please enter 'good' or 'bad' or 'skip'.");
            }
        }
    }    
    if count >= 2 || current_weekday == 7 {
        let average = total as f32 / count as f32;
        let week_color = if average > 0.0 {
            "\x1b[32m●\x1b[0m " // Green dot for good week
        } else if average < 0.0 {
            "\x1b[31m●\x1b[0m " // Red dot for bad week
        } else {
            "\x1b[37m●\x1b[0m " // Neutral color dot for neutral week
        };
        diagram.push_str(week_color);
    } else {
        diagram.push_str("\x1b[37m●\x1b[0m "); // Neutral color dot for current week
    }
    
    // Display the diagram
    println!("\n{}", diagram);
    
    // Display the number of weeks passed and remaining
    let total_weeks = 80 * 52; // Assuming an 80-year lifespan
    let weeks_remaining = total_weeks - weeks_passed;
    println!("You have lived {} weeks.", weeks_passed);
    println!("You have approximately {} weeks remaining.", weeks_remaining);
    
    user_responses.insert(weeks_passed, week_responses);
    user_responses
}

fn store_user_data(data: &UserData) {
    let json_data = serde_json::to_string_pretty(data).expect("Failed to serialize data");
    let file_name = format!("user_data_{}.json", Utc::now().timestamp());
    let mut file = File::create(file_name).expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write data to file");
}
