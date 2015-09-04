use std::io::prelude::*;
use std::fs::File;
use std::process;
use std::env;
use system_services::SystemInterface;
mod configuration;
mod provisioner;
mod logger;
mod file;
mod system_services;
mod platform;

fn main() {
    println!("sokan");
    let mut filename = "default.yaml".to_string();

    if env::args().len() > 1 {
        let mut args_iterator = env::args().skip(1);
        filename = args_iterator.next().unwrap().to_string();
    }

    let mut file_result :std::fs::File;
    match File::open(&filename) {
        Ok(result) => file_result = result,
        Err(error) => {
            println!("Error occured while accessing {}: {:?}", filename, error);
            return;
        }
    }

    let mut yaml_file = String::new();
    match file_result.read_to_string(&mut yaml_file) {
        Err(error) => println!("Failed to read {} - {:?}", filename, error),
        _ => { }
    }

    let configuration = configuration::from_yaml(yaml_file.to_string());
    let service = system_services::SystemServices;
    let provisioner = provisioner::Provisioner::new(service.clone());

    let exit_code = match service.platform() {
        system_services::OSType::redhat  => provisioner.provision(&configuration),
        _                                => 1
    };

    process::exit(exit_code);
}
