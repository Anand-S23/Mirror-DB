use std::io::{self, Write};

fn main() {
    let exit: String = String::from(".exit");


    loop {
        print!("mirror > "); 
        io::stdout().flush().unwrap();

        let mut input_buffer = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        input_buffer = input_buffer.trim().to_string();

        if input_buffer.eq(&exit) {
            break;
        } else {
            println!("Unrecognized command '{input_buffer}'\n")
        }
    }
}
