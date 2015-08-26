use std::io::prelude::*;
use std::fs::File;
use std::process;
mod configuration;
mod shell;
mod command;
mod output;
mod file;

fn main() {
    println!("sokan");
    let mut file_result :std::fs::File;
    match File::open("default.yaml") {
        Ok(result) => file_result = result,
        Err(error) => {
            println!("Error occured while accessing default.yaml: {:?}", error);
            return;
        }
    }

    let mut yaml_file = String::new();
    match file_result.read_to_string(&mut yaml_file) {
        Err(error) => println!("Failed to read default.yaml - {:?}", error),
        _ => { }
    }

    let configuration = configuration::from_yaml(yaml_file.to_string());
    let exit_code = shell::provision(&configuration);
    process::exit(exit_code);
}
