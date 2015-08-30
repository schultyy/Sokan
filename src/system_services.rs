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
