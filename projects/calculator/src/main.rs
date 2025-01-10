use std::io;

fn main() {
    loop {
        // Print the menu
        println!("Calculator Menu:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");
        println!("Enter your choice:");

        // Read the user's choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting the program.");
            break;
        }

        // Read the two numbers
        let (num1, num2) = (read_number("Enter the first number: "), read_number("Enter the second number: "));

        // Perform the selected operation
        match choice {
            1 => println!("Result: {}", num1 + num2),
            2 => println!("Result: {}", num1 - num2),
            3 => println!("Result: {}", num1 * num2),
            4 => {
                if num2 != 0.0 {
                    println!("Result: {}", num1 / num2)
                } else {
                    println!("Error: Division by zero is not allowed.");
                }
            },
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

// Function to read a number from the user
fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}


