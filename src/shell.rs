use std::process::Command;
use command;

pub fn run_command(command_str: String) -> String {
    let command = command::parse(command_str);

    let output_handle = Command::new(command.command)
        .args(&command.args)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let shell_output = String::from_utf8(output_handle.stdout).unwrap();
    shell_output
}
