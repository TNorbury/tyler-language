use std::collections::HashMap;

/// This struct will be used to store symbols as well as the state of the
/// program as it executes
pub struct ProgramState {
    lists: HashMap<String, Vec<String>>,
    execs: HashMap<String, String>,
    command_history: Vec<String>,
    repeat_level: usize,
}

/// Given a list of commands, the processor will process each command in order
pub fn process(commands: Vec<&str>) {
    // Create our symbol table
    let mut state: ProgramState = ProgramState {
        lists: HashMap::new(),
        execs: HashMap::new(),
        command_history: Vec::new(),
        repeat_level: 0,
    };

    // Process our commands
    for command in commands {
        if !command.is_empty() {
            process_command(command, &mut state);
        }
    }
}

/// Process the given command
fn process_command(command: &str, state: &mut ProgramState) {
    let command_split: Vec<&str> = command.split(" ").collect();

    // The command code will either be one of the five core, or an exec
    let command_code = command_split[0];
    let command_args = &command_split[1..command_split.len()];

    // Determine which type of command we're executing, a core command, or an
    // exec
    match command_code {
        "t" => tell(command_args, state),
        "y" => yell(command_args, state),
        "l" => list(command_args, state),
        "e" => exec(command_args, state),
        "r" => repeat(command_args, state),
        _ => process_exec(command_code, state),
    }

    state.command_history.push(command.to_string());
}

/// "Tell" the arguments
fn tell(args: &[&str], state: &mut ProgramState) {
    let to_tell: String;
    // If there is only one argument, see if it's a list or exec
    if args.len() == 1 {
        let arg = args[0];
        if state.lists.contains_key(arg) {
            to_tell = format!("{} => {:?}", arg, state.lists[arg]);
        }
        // Print the code of the exec
        else if state.execs.contains_key(arg) {
            to_tell = format!("{} => {}", arg, state.execs[arg]);
        }
        // If it's neither, we'll just print that value
        else {
            to_tell = arg.to_string();
        }
    } else {
        to_tell = args.join(" ");
    }
    println!("{}", to_tell);
}

/// "Yell" the arguments
fn yell(args: &[&str], state: &mut ProgramState) {
    let to_yell: String;

    // If there is only one argument, see if it's a list or exec
    if args.len() == 1 {
        let arg = args[0];
        if state.lists.contains_key(arg) {
            to_yell = format!("{} => {:?}", arg, state.lists[arg]);
        }
        // Print the code of the exec
        else if state.execs.contains_key(arg) {
            to_yell = format!("{} => {}", arg, state.execs[arg]);
        }
        // If it's neither, we'll just print that value
        else {
            to_yell = arg.to_string();
        }
    } else {
        to_yell = args.join(" ");
    }

    println!("{}", to_yell.to_uppercase());
}

/// Creates a list and put it into the symbol table
/// args should have a minimum of two arguments
///      1. The name of the list
///      2 - N. The items that'll go into this list
fn list(args: &[&str], state: &mut ProgramState) {
    if args.len() < 2 {
        panic!("l must be given a name for the list, and contents to put into said list");
    }

    let list_name = args[0].to_string();

    // Go over all the items for this list provided by the arguments, and put
    // them into the vector
    let mut list_items: Vec<String> = Vec::new();
    for item in &args[1..args.len()] {
        list_items.push(item.to_string());
    }

    // insert the vector into our symbol table
    state.lists.insert(list_name, list_items);
}

/// Create an executable and put it into the symbol table
/// args should have two arguments
///      1. The name of the exec
///      2. The code that the exec will represent
fn exec(args: &[&str], state: &mut ProgramState) {
    if args.len() < 2 {
        panic!("e must be given a name for the exec, and the code the exec represents");
    }

    // extract the exec name and code
    let exec_name = args[0].to_string();
    let exec_code = args[1..args.len()].join(" ").to_string();

    // and put it into the symbol table
    state.execs.insert(exec_name, exec_code);
}

/// Repeat, which can have one or none arguments
///      none: Repeat the last line of code
///      one: The name of a list. Repeat all the contents in the list
fn repeat(args: &[&str], state: &mut ProgramState) {
    // We use the repeat level to know how many repeats deep we are. This helps
    // when there are multiple repeats in a row and we need to know how far back
    // in the command history we
    state.repeat_level += 1;

    // If our repeat level is higher than the number of commands in our history,
    // then we'll panic
    if state.repeat_level > state.command_history.len() {
        can_not_repeat_panic();
    }

    let mut tab_prefix: String = "".to_string();

    for _i in 1..state.repeat_level {
        tab_prefix += "\t";
    }

    // If there were no arguments, then we'll just execute the last line of code
    if args.len() == 0 {
        match state
            .command_history
            .get(state.command_history.len() - state.repeat_level)
        {
            // Cloning the command so that it's no longer a reference to a command
            // in our symbol table
            Some(command) => process_command(command.clone().as_str(), state),

            // error if repeating on first line
            None => can_not_repeat_panic(),
        }
    }
    // if there was an argument provided, then we'll try to repeat the list
    else {
        let list_name = args[0];
        // verify that it's a list name, and if so "repeat" the list
        if state.lists.contains_key(list_name) {
            let list = &state.lists[list_name].clone();
            println!("{}Contents of {}:", tab_prefix, list_name);
            for item in list {
                // if the item is a list, print the name of the list, then repeat
                // said list
                if state.lists.contains_key(item) {
                    println!("\t{}{}", tab_prefix, item);
                    repeat(&[&item], state);
                }
                // if it's an exec, execute the exec
                else if state.execs.contains_key(item) {
                    process_exec(state.execs[item].clone().as_str(), state);
                }
                // otherwise, then it's just a value and we can print that
                else {
                    println!("\t{}{}", tab_prefix, item);
                }
            }
        }
        // if not, panic
        else {
            panic!(
                "ERROR: Tired to repeat a list ({}) that doesn't exist",
                list_name
            );
        }
    }

    state.repeat_level -= 1;
}

/// Processes an exec
fn process_exec(exec_name: &str, state: &mut ProgramState) {
    // Get the exec, if it exists
    match state.execs.get(exec_name) {
        // If the given name is a valid exec, then we'll process it
        Some(exec) => process_command(exec.clone().as_str(), state),

        // Otherwise, error
        None => panic!(
            "ERROR: Tried to run an exec ({}) that doesn't exist",
            exec_name
        ),
    }
}

/// This panic should be thrown in the event that a r command is on the first
/// line of the file
fn can_not_repeat_panic() {
    panic!("You can't repeat on the first line of a program");
}
