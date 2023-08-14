// The `std::io` module provides a suite of useful functions to handle input/output.
use std::io;

fn main() {
    // An infinite loop, ensuring our calculator keeps prompting the user for input until 'exit' is typed.
    loop {
        println!("Enter an expression (e.g. 5 + 3) or type 'exit' to quit:");

        // Declare a mutable variable `input` to hold the user's input.
        let mut input = String::new();

        // Read a line from standard input and store it in our `input` variable.
        io::stdin().read_line(&mut input).unwrap();

        // Trim any whitespace or newline characters from the start and end of the input string.
        let input = input.trim();

        // If the user types 'exit', break out of the loop, ending the program.
        if input == "exit" {
            break;
        }

        // Split the input string into individual "tokens" (parts) based on spaces.
        // For example, "5 + 3" becomes ["5", "+", "3"].
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // Check if we don't have exactly 3 tokens (number, operator, number). If not, prompt again.
        if tokens.len() != 3 {
            println!("Please enter a valid expression!");
            continue;
        }

        // Try to parse the first token as a floating-point number.
        let num1: f64 = match tokens[0].parse() {
            // If successful, store the number.
            Ok(val) => val,
            // If there's an error (e.g., it's not a number), show an error and ask again.
            Err(_) => {
                println!("Invalid number: {}", tokens[0]);
                continue;
            }
        };

        // Similarly, try to parse the third token as a floating-point number.
        let num2: f64 = match tokens[2].parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid number: {}", tokens[2]);
                continue;
            }
        };

        // Match the operator token to decide which operation to perform.
        let result = match tokens[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            // If the operator is not recognized, show an error and ask again.
            _ => {
                println!("Unsupported operation: {}", tokens[1]);
                continue;
            }
        };

        // Display the result of the calculation.
        println!("Result: {}", result);
    }
}
