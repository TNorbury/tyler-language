use std::env;
use std::fs;
use std::process;

mod language_processor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("You must provide a tyler file to execute!");
        process::exit(1);
    }
    let file = &args[1];
    println!("Executing file {}\n", file);

    // read in the file to execute, and create a vector of line
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    language_processor::process(lines);
}
