use std::process::Output;
use std::process::ExitStatus;

pub fn print_shellout(shellout: Output) {

    let exit_status = shellout.status;
    match exit_status.success() {
        true => {
            println!("{:?}", String::from_utf8_lossy(&shellout.stdout))
        },
        false => {
            println!("{:?}", String::from_utf8_lossy(&shellout.stderr))
        }
    }
}
