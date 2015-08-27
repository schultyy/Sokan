use std::process::{Command, Output};
use command;
use configuration;
use output;
use file;

pub fn provision(configuration: &configuration::Configuration) -> i32 {

    let mut packages = Vec::new();
    let mut commands = Vec::new();

    packages.extend(configuration.packages.iter().cloned());
    commands.extend(configuration.commands.clone());

    commands.reverse();
    packages.reverse();

    let mut exit_codes = Vec::new();

    while let Some(cmd) = commands.pop() {
        let shellout = run(&cmd);
        output::print_shellout::<command::Command>(&cmd, &shellout);
        let exit_status = shellout.status.clone();
        if exit_status.success() == false {
            exit_codes.push(exit_status.code().unwrap());
        }
    }

    let package_exit_codes = install_packages(&packages);
    exit_codes.extend(&package_exit_codes[..]);

    let results = handle_file_resources(&configuration.files);
    exit_codes.extend(&results[..]);

    exit_codes.sort();

    match exit_codes.last() {
        Some(&0) => return 0,
        None     => return 0,
        _        => return 1
    }
}

fn install_packages(package_list: &Vec<String>) -> Vec<i32> {
    let mut exit_codes = Vec::new();
    let mut packages = package_list.clone();

    while let Some(package) = packages.pop() {
        if is_package_installed(&package) {
            continue;
        }
        let shellout = install_package(&package);
        output::print_shellout::<String>(&package, &shellout);
        let exit_status = shellout.status.clone();
        if exit_status.success() == false {
            exit_codes.push(exit_status.code().unwrap());
        }
    }
    exit_codes
}

fn handle_file_resources(resources: &Vec<file::FileResource>) -> Vec<i32> {
    let mut results = Vec::new();
    for resource in resources {
        let result = resource.write_file();
        match result {
            Ok(_) =>  {
                let msg = format!("==> Wrote file {}", resource.path);
                output::print_message(msg, output::MessageType::Stdout);
                results.push(0);
            },
            Err(err) => {
                let msg = format!("==> Error while writing file {}", resource.path);
                output::print_message(msg, output::MessageType::Stderr);
                output::print_message(format!("==> {}", err), output::MessageType::Stderr);
                results.push(1);
            }
        }
    }
    results
}

fn is_package_installed(package_name: &String) -> bool {
    let command_str = format!("yum list installed {}", package_name);
    let output = Command::new(command_str).output();
    match output {
        Ok(_) => return true,
        Err(_)    => return false
    }
}

fn install_package(package: &String) -> Output {
    Command::new("sudo")
        .args(&vec!["yum", "install", "-y", package])
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
}

fn run(command: &command::Command) -> Output {
    let output_handle = Command::new(command.command.to_string())
        .args(&command.args)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    output_handle
}
