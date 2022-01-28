use std::env::args;
mod calc;

fn main() {
    let args: Vec<String> = args().collect();
    let f: i32 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please input a number!");
            std::process::exit(1);
        }
    };
    let c: i32 = calc::conv(&f);
    println!("Converting {} fahrenheit to celcius.", f);
    println!("{}", c);
}
