use std::process::Command;

pub fn run_command(command: String) {
    let mut fragments: Vec<&str> = command.split_whitespace().collect();
    let cmd = fragments.pop().unwrap();
    let arguments = &fragments[..];

    let output_handle = Command::new(cmd)
        .args(arguments)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let shell_output = String::from_utf8(output_handle.stdout).unwrap();
    println!("{:?}", shell_output.to_string());
}
