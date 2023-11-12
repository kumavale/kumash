use anyhow::Context;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
enum Token<'a> {
    Command(&'a str),
    Pipe,
    Redirect,
}

fn parse(input: &str) -> Vec<Token<'_>> {
    let tokens = input.split_inclusive(|c| c == '|' || c == '>');
    tokens
        .map(|t| if t.ends_with("|") {
            vec![]
        } else if t.ends_with(">") {
        } else {
        }
            "|" => Token::Pipe,
            ">" => Token::Redirect,
            _ => Token::Command(t),
        })
        .collect()
}

fn main() {
    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read line
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        // Parse input line
        let mut commands = parse(&input_line).into_iter().peekable();
        eprintln!("{commands:?}");

        let mut previous_process = None;
        while let Some(token) = commands.next() {
            let command = match token {
                Token::Command(s) => s.split_whitespace(),
                _ => {
                    eprintln!("unexpected token: {token:?}");
                    break;
                }
            };

            // Execute
            let stdout = match commands.peek() {
                Some(Token::Pipe) => {
                    commands.next();
                    Stdio::piped()
                }
                Some(Token::Redirect) => todo!(),
                Some(Token::Command(s)) => unreachable!("{s}"),
                None => Stdio::inherit(),
            };
            match execute(command, previous_process, stdout) {
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
) -> anyhow::Result<Child>
where
    I: Iterator<Item = &'a str>,
{
    let command = tokens.next().context("no command entered")?;
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
                .spawn()?;
            Ok(child)
        }
    }
}
