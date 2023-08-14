## Project 1: A simple calculator

### 1.Installation and Setup
To install Rust on your system, run:
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh```

### 2.During the installation, you will be presented with three options. 
Choose:
```1) Proceed with installation (default)```

### 3.After the installation completes, activate Rust by entering:
```source "$HOME/.cargo/env"```

### 4.Verify the installation by checking Rust's version:
```rustc --version```

### 5.Create a new Rust project named "simple_calculator":
```cargo new simple_calculator```

### 6.Navigate to the project's directory:
```cd simple_calculator```

### Description of the Project:

This Rust-based calculator is a simple command-line tool. When the user runs the program, it prompts them to input a mathematical expression in the format number operator number (e.g., 5 + 3). The program then processes this input, breaks it down into its components (number1, operator, number2), performs the specified arithmetic operation, and displays the result.

Key features and behaviors include:

The ability to handle basic arithmetic operations: addition, subtraction, multiplication, and division.
Error handling for invalid numbers and unsupported operators.
The option for users to exit the program by typing exit.
The program showcases fundamental Rust concepts such as variable declaration, user input/output, basic arithmetic operations, string manipulation, and pattern matching.