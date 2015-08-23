use std::process::Output;
use std::process::ExitStatus;
use std::fmt::Display;
extern crate term;
use std::io::prelude::*;

pub fn print_shellout<T: Display>(thing: &T, shellout: &Output) {
    println!("Executed {}", thing);
    print_output(&shellout);
}

fn print_output(shellout: &Output) {
    let exit_status = shellout.status;
    let mut stdout_terminal = term::stdout().unwrap();
    let mut stderr_terminal = term::stderr().unwrap();
    stderr_terminal.fg(term::color::RED).unwrap();
    stdout_terminal.fg(term::color::GREEN).unwrap();

    match exit_status.success() {
        true => {
            let stdout_str = format!("==> {:?}", String::from_utf8_lossy(&shellout.stdout));
            writeln!(stdout_terminal, "{}", stdout_str).unwrap();
        },
        false => {
            let stderr_str = format!("==> {:?}", String::from_utf8_lossy(&shellout.stderr));
            writeln!(stderr_terminal, "{}", stderr_str);
        }
    }
    stderr_terminal.reset().unwrap();
    stdout_terminal.reset().unwrap();
}
