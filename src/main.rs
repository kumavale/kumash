use anyhow::anyhow;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    env_logger::init();

    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read line
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        // Parse input line
        let tokens = input_line.split_whitespace().collect::<Vec<&str>>();
        if tokens.is_empty() {
            continue;
        }

        // Execute
        if let Err(e) = execute(tokens) {
            log::error!("fatal error: {e}");
        }
    }
}

fn execute(tokens: Vec<&str>) -> anyhow::Result<i32> {
    if tokens[0] == "exit" {
        println!("(^-^)/~~");
        std::process::exit(0);
    }

    let bin = tokens[0];
    let args = if tokens.len() == 1 { &[] } else { &tokens[1..] };
    let status = Command::new(bin).args(args).status()?;
    status.code().ok_or(anyhow!("Process terminated by signal"))
}
