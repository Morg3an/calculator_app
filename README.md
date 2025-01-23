# Calculator App

## Overview

This project is part of the **Day 3** learning challenge in the 30-Day Rust Challenge. The calculator app demonstrates how to use functions, parameters, return types, and modules to organize code effectively.

## Features

- Perform basic mathematical operations:
  - **Addition**
  - **Subtraction**
  - **Multiplication**
  - **Division**
- Organized into modules for cleaner structure and maintainability.
- Provides error handling for invalid input and division by zero.

## Concepts Covered

- **Functions**: Define reusable blocks of code for each operation.
- **Modules**: Organize operations into separate files.
- **Control Flow**: Use `match` to handle user choices.
- **Error Handling**: Manage invalid inputs gracefully.

## Project Structure

```plaintext
src/
├── main.rs
├── calculator/
    ├── mod.rs
    ├── add.rs
    ├── subtract.rs
    ├── multiply.rs
    ├── divide.rs
```

## How to Run
1. Clone the repository:
   ```bash
   git clone https://github.com/Morg3an/calculator_app.git
   ```
2. Navigate to the project directory:
   ```bash
   cd calculator_app
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
   cargo run
   ```

## Usage
1. Run the app and select an option by entering a number:
    ```
    1. Add
    2. Subtract
    3. Multiply
    4. Divide
    ```
2. Enter the first and second numbers when prompted
3. View the result in the console.

## Example
    ```
    Calculator App
    Select an operation:
    1. Add
    2. Subtract
    3. Multiply
    4. Divide
    1
    Enter the first number:
    5
    Enter the second number:
    3
    The result is: 8
    ```

## Technologies Used
- Rust
- Cargo (Rust's package manager)

## Edge Cases Handled
- Invalid inputs: The app ensures that only valid numbers are processed.
- Division by zero: Prevents crashes and notifies the user.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)