use std::io::prelude::*;
use std::fs::File;
use std::process;
mod configuration;
mod shell;
mod command;
mod output;

fn main() {
    println!("sokan");
    let mut fileResult :std::fs::File;
    match File::open("default.yaml") {
        Ok(result) => fileResult = result,
        Err(error) => {
            println!("Error occured while accessing default.yaml: {:?}", error);
            return;
        }
    }

    let mut yaml_file = String::new();
    match fileResult.read_to_string(&mut yaml_file) {
        Err(error) => println!("fail"),
        _ => { }
    }

    let mut configuration = configuration::from_yaml(yaml_file.to_string());
    configuration.commands.reverse();

    while let Some(cmd) = configuration.commands.pop() {
        let shellout = shell::run_command(&cmd);
        output::print_shellout(&cmd, &shellout);
        let exit_status = shellout.status.clone();
        if exit_status.success() == false {
            let code = exit_status.code().unwrap();
            process::exit(code);
        }
    }
}
