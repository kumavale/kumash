use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read line
        let input = read_input();

        // Parse input line
        let mut commands = input.split(" | ").peekable();

        let mut previous_process = None;
        while let Some(command) = commands.next() {
            let tokens = command.split_whitespace();

            // Execute
            let stdout = commands.peek().map_or(Stdio::inherit(), |_| Stdio::piped());
            match execute(tokens, previous_process, stdout) {
                Ok(child) => previous_process = Some(child),
                Err(e) => {
                    eprintln!("kumash error: {e}");
                    previous_process = None;
                    break;
                }
            }
        }

        if let Some(final_process) = previous_process {
            match final_process.wait_with_output() {
                Ok(output) => io::stdout().write_all(&output.stdout).unwrap(),
                Err(e) => eprintln!("kumash error: {e}"),
            }
        }
    }
}

fn execute<'a, I>(
    mut tokens: I,
    previous_process: Option<Child>,
    stdout: Stdio,
) -> Result<Child, &'static str>
where
    I: Iterator<Item = &'a str>,
{
    let Some(command) = tokens.next() else {
        return Err("no command entered");
    };
    match command {
        "exit" => {
            println!("(^-^)/~~");
            std::process::exit(0);
        }
        _ => {
            let stdin = previous_process.map_or(Stdio::inherit(), |child: Child| {
                Stdio::from(child.stdout.unwrap())
            });
            let child = Command::new(command)
                .args(tokens)
                .stdin(stdin)
                .stdout(stdout)
                .spawn()
                .unwrap();
            Ok(child)
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
