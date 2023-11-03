use std::io::{self, Write};

use crate::parser;

#[derive(Debug, Default)]
pub struct Shell;

impl Shell {
    pub fn start(&self) -> ! {
        loop {
            // Prompt
            print!("$ ");
            io::stdout().flush().unwrap();

            // Read line
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line)
                .unwrap();

            // Parse input line
            let command = parser::parse_input_line(&input_line);

            // Execute
            let status = self.execute(command);

            // Postprocess
            // TODO
        }
    }

    fn execute(&self, command: Vec<&str>) -> () {
        match command[0] {
            "exit" => std::process::exit(0),
            input => println!("{input}"),
        }
    }
}
