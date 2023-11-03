use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, getpid, ForkResult};
use std::ffi::CString;
use std::io::{self, Write};

use crate::parser;

#[derive(Debug, Default)]
pub struct Shell {}

impl Shell {
    pub fn start(&self) -> ! {
        let mut prompt = String::from("$ ");

        loop {
            // Prompt
            print!("{prompt}");
            io::stdout().flush().unwrap();

            // Read line
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();

            // Parse input line
            let command = parser::parse_input_line(&input_line);

            // Execute
            if command.is_empty() {
                continue;
            }
            let status = self.execute(command);

            // Postprocess
            match status {
                Ok(0) => prompt = String::from("$ "),
                Ok(code) => prompt = format!("[{code}]$ "),
                Err(e) => log::error!("fatal error: {e}"),
            }
        }
    }

    fn execute(&self, command: Vec<&str>) -> Result<i32, i32> {
        if command[0] == "exit" {
            println!("(^-^)/~~");
            std::process::exit(0);
        }

        let command = command
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect::<Vec<_>>();
        let filename = command[0].as_c_str();

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                log::debug!("PID: parent({})", getpid());
                match waitpid(child, None) {
                    Ok(stat) => match stat {
                        WaitStatus::Exited(_, exitcode) => Ok(exitcode),
                        _ => Ok(1),
                    },
                    Err(_) => {
                        log::error!("waitpid failed");
                        Err(1)
                    }
                }
            }
            Ok(ForkResult::Child) => {
                log::debug!("PID: child({})", getpid());
                let exitcode = if let Err(e) = execvp(filename, &command) {
                    e as i32
                } else {
                    0
                };
                std::process::exit(exitcode);
            }
            Err(_) => {
                log::error!("fork failed");
                Err(1)
            }
        }
    }
}
