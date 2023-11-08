use std::{io::{self, Write}, process::exit};

enum CommandRunStatus {
    Success,
    Unrecognized
}

const COMMAND_EXIT: &str = ".exit";

fn main() {
    loop {
        print!("mirror > "); 
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        input_buffer = input_buffer.trim().to_string();
        if input_buffer.len() != 0 && input_buffer.chars().nth(0).expect("Could not parse command") == '.' {
            match run_command(&input_buffer) {
                CommandRunStatus::Success => continue,
                CommandRunStatus::Unrecognized => {
                    println!("Unrecognized command '{input_buffer}'\n");
                    continue;
                },
            }
        }
        
        println!("Test Coverage"); 
    }
}

fn run_command(command: &String) -> CommandRunStatus {
    if command.eq(&COMMAND_EXIT.to_string()) {
        exit(0);
    } 

    CommandRunStatus::Unrecognized
}

