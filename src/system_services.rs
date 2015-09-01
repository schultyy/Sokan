use std::process::{Command, Output};
use std::fs;
use std::io::prelude::*;
use std::fs::File;

pub struct SystemServices;

pub trait SystemInterface {
    fn file_exists(&self, path: &String) -> bool;
    fn read_file(&self, path: &String) -> Option<String>;
    fn is_package_installed(&self, package_name: &String) -> bool;
    fn install_package(&self, package: &String) -> Output;
    fn get_hostname(&self) -> Option<String>;
    fn set_hostname(&self, new_hostname: &String) -> bool;
}

impl SystemInterface for SystemServices {
     fn file_exists(&self, path: &String) -> bool {
        let metadata = fs::metadata(path);

        match metadata {
            Ok(md) => md.is_dir() || md.is_file(),
            Err(_) => false
        }
    }

     fn read_file(&self, path: &String) -> Option<String> {
        let mut file_handle = File::open(path);
        let mut s = String::new();
        match file_handle {
            Ok(mut handle) => {
                handle.read_to_string(&mut s);
                Some(s)
            },
            Err(err)   => None
        }
    }

     fn is_package_installed(&self, package_name: &String) -> bool {
        let command_str = format!("yum list installed {}", package_name);
        let output = Command::new(command_str).output();
        output.is_ok()
    }

     fn install_package(&self, package: &String) -> Output {
        Command::new("yum")
            .args(&vec!["install", "-y", package])
            .output()
            .unwrap()
    }

     fn get_hostname(&self) -> Option<String> {
        let output = Command::new("hostname").output().unwrap();
        if output.status.success() {
            let hostname_with_newline = String::from_utf8(output.stdout).unwrap();
            Some(hostname_with_newline.replace("\n", ""))
        } else {
            None
        }
    }

     fn set_hostname(&self, new_hostname: &String) -> bool {
        let output = Command::new("hostname")
            .arg(new_hostname)
            .output()
            .unwrap();
        output.status.success()
    }
}
