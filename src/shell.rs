use std::process::Command;
use std::process::Output;
use command;

pub fn run_command(command_str: String) -> Output {
    let command = command::parse(command_str);

    let output_handle = Command::new(command.command)
        .args(&command.args)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    output_handle
}
