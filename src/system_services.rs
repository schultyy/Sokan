use std::process::{Command, Output};
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use platform;

#[derive(Clone)]
pub struct SystemServices;

pub enum OSType {
    Redhat,
    Unknown
}

pub trait SystemInterface {
    fn file_exists(&self, path: &String) -> bool;
    fn read_file(&self, path: &String) -> Option<String>;
    fn is_package_installed(&self, package_name: &String) -> bool;
    fn install_package(&self, package: &String) -> Output;
    fn get_hostname(&self) -> Option<String>;
    fn set_hostname(&self, new_hostname: &String) -> bool;
    fn os_type(&self) -> OSType;
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
        let file_handle = File::open(path);
        let mut s = String::new();
        match file_handle {
            Ok(mut handle) => {
                if handle.read_to_string(&mut s).is_ok() {
                    Some(s)
                } else {
                    None
                }
            },
            Err(_)   => None
        }
    }

     fn is_package_installed(&self, package_name: &String) -> bool {
        let platform = platform::for_os_type(self.os_type()).unwrap();
        let command_str = format!("{} {}", platform.package_installed_command, package_name);
        let cmd_list :Vec<&str> = command_str.split_whitespace().collect();
        let cmd_name :String = cmd_list.first().unwrap().to_string();
        let mut cmd_args = Vec::new();
        cmd_args.extend(cmd_list.iter().skip(1).map(|s| s.to_string()).collect::<Vec<_>>());

        match Command::new(cmd_name).args(&cmd_args[..]).output() {
            Ok(output) => output.status.success(),
            Err(_)   => false
        }
    }

     fn install_package(&self, package: &String) -> Output {
        let platform = platform::for_os_type(self.os_type()).unwrap();
        let cmd_list :Vec<&str> = platform.install_command.split_whitespace().collect();
        let cmd_name :String = cmd_list.first().unwrap().to_string();
        let mut cmd_args = Vec::new();
        cmd_args.extend(cmd_list.iter().skip(1).map(|s| s.to_string()).collect::<Vec<_>>());
        cmd_args.push(package.to_string());

        Command::new(cmd_name)
            .args(&cmd_args[..])
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

    fn os_type(&self) -> OSType {
        if self.file_exists(&"/etc/redhat-release".to_string()) ||
            self.file_exists(&"/etc/centos-release".to_string()){
                OSType::Redhat
            } else {
                OSType::Unknown
            }
    }
}
