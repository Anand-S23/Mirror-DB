use std::{io::{self, Write}, process::exit};

enum CommandResult {
    Success,
    Unrecognized
}

enum Statement {
    None,
    Insert,
    Select
}

const COMMAND_EXIT: &str = ".exit";
const STATEMENT_INSERT: &str = "insert";
const STATEMENT_SELECT: &str = "select";

fn main() {
    loop {
        print!("mirror > "); 
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        input_buffer = input_buffer.trim().to_string();
        if input_buffer.len() == 0 {
            continue;
        }

        if input_buffer.chars().nth(0).expect("Could not parse command") == '.' {
            match execute_command(&input_buffer) {
                CommandResult::Success => continue,
                CommandResult::Unrecognized => {
                    println!("Unrecognized command '{input_buffer}'");
                    continue;
                },
            }
        }

        let (ok, statement) = prepare_statement(&input_buffer);
        if !ok {
            println!("Unrecognized keyword at the start of '{input_buffer}'");
            continue;
        }

        execute_statement(statement);
    }
}

fn execute_command(command: &String) -> CommandResult {
    match command.as_str() {
        COMMAND_EXIT => exit(0),
        _ => CommandResult::Unrecognized
    }
}

fn prepare_statement(statement: &String) -> (bool, Statement) {
    let split_statement: Vec<&str> = statement.split_whitespace().collect();
    match split_statement[0] {
        STATEMENT_INSERT => (true, Statement::Insert),
        STATEMENT_SELECT => (true, Statement::Select),
        _ => (false, Statement::None)
    }
}

fn execute_statement(statement: Statement) {
    match statement {
        Statement::Insert => println!("Inserting..."),
        Statement::Select => println!("Selecting..."),
        // TODO: Maybe remove None statement
        Statement::None => panic!("Some statment is invalid, should not be possible")
    }
}

