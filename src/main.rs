use kumash::shell::Shell;

fn main() {
    env_logger::init();

    let shell = Shell::default();
    shell.start();
}
