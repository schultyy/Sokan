use std::process::Output;
use std::process::ExitStatus;
use std::fmt::Display;

pub fn print_shellout<T: Display>(thing: &T, shellout: &Output) {
    println!("Executed {}", thing);
    print_output(&shellout);
}

fn print_output(shellout: &Output) {
    let exit_status = shellout.status;
    match exit_status.success() {
        true => {
            println!("==> {:?}", String::from_utf8_lossy(&shellout.stdout))
        },
        false => {
            println!("==> {:?}", String::from_utf8_lossy(&shellout.stderr))
        }
    }
}
