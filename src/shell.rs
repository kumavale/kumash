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
            if status == 0 {
                prompt = String::from("$ ");
            } else {
                prompt = format!("[{status}]$ ");
            }
        }
    }

    fn execute(&self, command: Vec<&str>) -> i32 {
        if command[0] == "exit" {
            println!("(^-^)/~~");
            std::process::exit(0);
        }

        let command = command
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect::<Vec<_>>();
        let bin = command[0].as_c_str();
        let args = command.iter().map(|s| s.as_c_str()).collect::<Vec<_>>();

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                log::debug!("PID: parent({})", getpid());
                match waitpid(child, None) {
                    Ok(stat) => match stat {
                        WaitStatus::Exited(_, exitcode) => exitcode,
                        _ => 1,
                    },
                    Err(_) => {
                        log::error!("waitpid failed");
                        0
                    }
                }
            }
            Ok(ForkResult::Child) => {
                log::debug!("PID: child({})", getpid());
                if let Err(e) = execvp(bin, &args) {
                    std::process::exit(e as i32);
                } else {
                    0
                }
            }
            Err(_) => {
                log::error!("fork failed");
                1
            }
        }
    }
}
