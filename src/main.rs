use std::io;

fn main() {
    println!("Simple Addition Calculator");

    println!("Enter the first number:");
    let num1: f64 = get_input();
    println!("Enter the second number:");
    let num2: f64 = get_input();

    let result = num1 + num2;
    println!("Result: {}", result);
}

fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
