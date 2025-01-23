// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// Import the calculator modules
mod calculator;

use std::io;

fn main() {
    println!("Calculator App");
    println!("Select an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number.");

    println!("Enter the first number: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");

    println!("Enter the second number:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    // Match the user's choice with the desired function
    let result = match choice {
        1 => calculator::add::add(a, b),
        2 => calculator::subtract::subtract(a, b),
        3 => calculator::multiply::multiply(a, b),
        4 => calculator::divide::divide(a, b),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    println!("The result is {}", result);
}
