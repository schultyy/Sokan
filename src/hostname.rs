use std::process::Command;

pub fn get() -> Option<String> {
    let output = Command::new("hostname").output().unwrap();
    if output.status.success() {
        let hostname_with_newline = String::from_utf8(output.stdout).unwrap();
        Some(hostname_with_newline.replace("\n", ""))
    } else {
        None
    }
}

pub fn set(new_hostname: &String) -> bool {
    let current_hostname = get();
    match current_hostname {
        Some(hn) => {
            if hn == new_hostname.to_string() {
                return true
            } else {
                let output = Command::new("hostname")
                    .arg(new_hostname)
                    .output()
                    .unwrap();
                output.status.success()
            }
        },
        None => false
    }
}
