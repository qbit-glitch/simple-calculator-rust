// Take input from user: 2 numbers and 1 string
use std::io;

#[derive(Debug)]
enum Calculator { 
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 },
}

impl Calculator {
    fn calculate(&self) -> f64 {
        match self {
            Calculator::Add { x, y } => x + y,
            Calculator::Subtract { x, y } => x - y,
            Calculator::Multiply { x, y } => x * y,
            Calculator::Divide { x, y } => {
                if *y == 0.0 {
                    panic!("Error: Division by zero!");
                } else {
                    x / y
                }
            },
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter 2 numbers along with arithmetic operation in this format: <num1> <num2> <operation>");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Check if we received exactly 3 inputs
    if parts.len() != 3 {
        println!("Invalid number of inputs");
        return;
    }

    // Parse the first number
    let num1 = match parts[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Not entered 1st number");
            return;
        }
    };

    // Parse the second number
    let num2 = match parts[1].parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Not entered 2nd number");
            return;
        }
    };

    // Get the operator and convert it to lowercase
    let op = parts[2].to_lowercase();

    // Match the operator and perform the calculation
    match op.as_str() {
        "add" => {
            let input_struct = Calculator::Add { x: num1, y: num2 };
            let result = input_struct.calculate();
            println!("Addition Result: {}", result);
        },
        "subtract" => {
            let input_struct = Calculator::Subtract { x: num1, y: num2 };
            let result = input_struct.calculate();
            println!("Subtraction Result: {}", result);
        },
        "multiply" => {
            let input_struct = Calculator::Multiply { x: num1, y: num2 };
            let result = input_struct.calculate();
            println!("Multiplication Result: {}", result);
        },
        "divide" => {
            let input_struct = Calculator::Divide { x: num1, y: num2 };
            let result = input_struct.calculate();
            println!("Division Result: {}", result);
        },
        _ => {
            println!("Invalid operator");
        }
    }
}
