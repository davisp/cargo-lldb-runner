use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    if std::env::args().len() < 2 {
        panic!("No command provided to execute.")
    }

    let mut cmd: Vec<String> = Vec::new();

    // Check if we're wanting to run in lldb
    if std::env::var("DEBUG").is_ok() {
        cmd.push("lldb".to_owned());
    }

    for arg in std::env::args().skip(1) {
        cmd.push(arg);
    }

    assert!(!cmd.is_empty());

    // Execute the subcommand.
    Command::new(cmd[0].clone()).args(&cmd[1..]).exec();
}
