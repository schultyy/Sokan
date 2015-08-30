use std::process::{Command, Output};
use std::fs;

pub fn file_exists(path: &String) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false
    }
}

pub fn is_package_installed(package_name: &String) -> bool {
    let command_str = format!("yum list installed {}", package_name);
    let output = Command::new(command_str).output();
    output.is_ok()
}

pub fn install_package(package: &String) -> Output {
    Command::new("yum")
        .args(&vec!["install", "-y", package])
        .output()
        .unwrap()
}

pub fn get_hostname() -> Option<String> {
    let output = Command::new("hostname").output().unwrap();
    if output.status.success() {
        let hostname_with_newline = String::from_utf8(output.stdout).unwrap();
        Some(hostname_with_newline.replace("\n", ""))
    } else {
        None
    }
}

pub fn set_hostname(new_hostname: &String) -> bool {
    let current_hostname = get_hostname();
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
