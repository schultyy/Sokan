use std::io::prelude::*;
use std::fs::File;
mod configuration;

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

    let commands = configuration::from_yaml(yaml_file.to_string());
    println!("{:?}", commands);
}
