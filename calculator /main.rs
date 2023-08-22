use std::io;

fn main() {
    println!("What's your number 1?");
    
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Failed to read line");
    let number1_input: f64 = number1.trim().parse().expect("Invalid number");

    println!("What's your operation?");
    
    let mut operation_sign = String::new();
    io::stdin().read_line(&mut operation_sign).expect("Failed to read line");
    let ope_sign_input: String = operation_sign.trim().to_lowercase(); // Convert to lowercase

    println!("What's your number 2?");
    
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Failed to read line");
    let number2_input: f64 = number2.trim().parse().expect("Invalid number");
    
    let operation = match ope_sign_input.as_str() {
        "add" | "+"  => Operation::Add { x: number1_input, y: number2_input },
        "subtract" | "-" => Operation::Subtract { x: number1_input, y: number2_input },
        "multiply" | "*" => Operation::Multiply { x: number1_input, y: number2_input },
        "divide" | "/" => Operation::Divide { x: number1_input, y: number2_input },
        _ => {
            println!("Invalid operation");
            return;
        }
    };
   
    let result = calculate(operation);
    println!("Result: {}", result);
}

enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 }, 
    Divide { x: f64, y: f64 },
}

fn calculate(equ: Operation) -> f64 {
    match equ {
        Operation::Add { x, y } => x + y,
        Operation::Subtract { x, y } => x - y,
        Operation::Multiply { x, y } => x * y,
        Operation::Divide { x, y } => x / y,
    }
}
