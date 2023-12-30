use clap::Parser;
use std::io::{BufRead, Lines, Stdin, StdinLock};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from(""))]
    name: String,
}

fn main() {
    // TODO: implement next session - https://github.com/mikaelmello/inquire

    let args: Args = Args::parse();

    println!("Welcome to 🧪");

    let mut name: String = args.name.clone();

    if name.is_empty() {
        println!("What's your name?");
        let stdin: Stdin = std::io::stdin();
        let mut iterator: Lines<StdinLock<'_>> = stdin.lock().lines();
        name = iterator.next().unwrap().unwrap();

        println!("Hello, {}!", name);
    } else {
        println!("Hello, {}!", name)
    }
}
