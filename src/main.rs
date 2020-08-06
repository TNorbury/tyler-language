use clap::{App, Arg};
use std::fs;

mod language_processor;

fn main() {
    const TYLER_FILE_KEY: &str = "FILE";

    // Set up the command line arguments
    let cl_args = App::new("tyler")
        .version("0.1.0")
        .author("tnorbury")
        .about("Interpreter for the tyler language")
        .arg(
            Arg::new(TYLER_FILE_KEY)
                .about("The tyler file to run")
                .value_name("tyler_file")
                .required(true)
                .index(1),
        )
        .get_matches();

    // extract the tyler file from the command line arguments
    let file = cl_args.value_of(TYLER_FILE_KEY).unwrap();
    println!("Executing file {}\n", file);

    // read in the file to execute, and create a vector of line
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    language_processor::process(lines);
}
