use std::collections::HashMap;

// This struct will be used to store symbols as the program executes
pub struct Symbols {
    lists: HashMap<String, Vec<String>>,
    execs: HashMap<String, String>,
    command_history: Vec<String>,
}

//  Given a list of commands, the processor will process each command in order
pub fn process(commands: Vec<&str>) {
    // println!("{:?}", commands);

    let mut symbols: Symbols = Symbols {
        lists: HashMap::new(),
        execs: HashMap::new(),
        command_history: Vec::new(),
    };

    //
    for command in commands {
        let command_split: Vec<&str> = command.split(" ").collect();

        // The command code will either be one of the five core, or an exec
        let command_code = command_split[0];
        let command_args = &command_split[1..command_split.len()];

        // Determine which type of command we're executing, a core command, or an
        // exec
        match command_code {
            "t" => tell(command_args),
            "y" => yell(command_args),
            "l" => println!("list"),
            "e" => println!("exec"),
            "r" => println!("repeat"),
            _ => println!("Not a core command"),
        }

        symbols.command_history.push(command.to_string());
    }
}

// "Tell" the arguments
// TODO: print out lists and execs
fn tell(args: &[&str]) {
    let to_tell = args.join(" ");
    println!("{}", to_tell);
}

// "Yell" the arguments
// TODO: print out lists and execs
fn yell(args: &[&str]) {
    let to_yell: String = args.join(" ").to_uppercase();
    println!("{}", to_yell);
}

/*
fn list() {}

fn exec() {}

fn repeat() {}
*/
