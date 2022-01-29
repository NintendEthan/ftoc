extern crate clap;
use clap::Parser;
mod calc;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    farenheit: i32,
}

fn main() {
    let cli = Cli::parse();

    let c: i32 = calc::conv(&cli.farenheit);
    println!("Converting {} fahrenheit to celcius.", cli.farenheit);
    println!("{}", c);
}
