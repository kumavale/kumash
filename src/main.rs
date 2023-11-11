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
        let tokens = input_line.split_whitespace();

        // Execute
        if let Err(e) = execute(tokens) {
            log::error!("fatal error: {e}");
        }
    }
}

fn execute<'a, I>(mut tokens: I) -> anyhow::Result<()>
where
    I: Iterator<Item = &'a str>,
{
    let command = match tokens.next() {
        Some(command) => command,
        None => return Ok(()),
    };

    match command {
        "exit" => {
            println!("(^-^)/~~");
            std::process::exit(0);
        }
        _ => {
            Command::new(command).args(tokens).spawn()?.wait()?;
        }
    }
    Ok(())
}
