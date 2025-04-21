use std::io;

fn main() {
    println!("What Celsius value do you want to convert to Fahrenheit?: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let x:i32 = input.trim().parse().expect("Please enter a valid number");

    let z = (x * 9 / 5) + 32;

    println!("{} Celsius to Fahrenheit is: {}", x, z);
}