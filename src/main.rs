use std::io;
use std::io::{stdout, Write};

fn main() {
    print!("Enter the value: ");
    stdout().flush();
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Error reading the line");
    let value: u32 = value.trim().parse().unwrap();
    print!("Celsius: C / Fahrenheit: F");
    print!("Select: ");
    stdout().flush();
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Error reading the line");
    let unit = unit.trim();

    if unit == "C" {
        println!("{value} Celsius -> {} Fahrenheit", (value as f64 * 1.8) + 32.0);
    } else if unit == "F" {
        println!("{value} Fahrenheit -> {} Celsius", (value as f64 - 32.0) * 0.55);
    } else {
        println!("unit: wrong here!");
    }
}
