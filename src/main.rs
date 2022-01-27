use std::io::*;
use text_io::read;

fn main() {
    println!("Converting celcius to fahrenheit.");
    print!("Fahrenheit: ");

    stdout().flush().unwrap();

    let f: i32 = read!();
    let c = ((f - 32) * 5) / 9;

    println!("Celcius: {}", c);
}
