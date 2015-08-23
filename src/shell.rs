use std::process::{Command, Output};
use command;
use configuration;
use output;

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

    exit_codes.sort();

    match exit_codes.last() {
        Some(&0) => return 0,
        _       => return 1
    }
}

fn is_package_installed(package_name: &String) -> bool {
    let command_str = format!("yum list installed {}", package_name);
    let output = Command::new(command_str)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    return output.status.success();
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
