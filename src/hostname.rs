use std::process::Command;
pub fn set(new_hostname: &String) -> bool {
    Command::new("hostname")
        .arg(new_hostname)
        .output()
        .is_ok()
}
