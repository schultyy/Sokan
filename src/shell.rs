use std::process::{Command, Output};
use command;

pub fn run_command(command: &command::Command) -> Output {
    let output_handle = Command::new(command.command.to_string())
        .args(&command.args)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    output_handle
}
