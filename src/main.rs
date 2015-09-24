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

fn is_supported() -> bool {
    let service = system_services::SystemServices;
    match service.os_type() {
        system_services::OSType::Redhat => true,
        _ => false
    }
}

fn main() {
    println!("sokan");
    let mut filename = "default.yaml".to_string();

    if env::args().len() < 2 {
        println!("No arguments provided");
        println!("Usage");
        println!("sokan <configuration_file>");
        process::exit(2);
    }
    let mut args_iterator = env::args().skip(1);
    filename = args_iterator.next().unwrap().to_string();

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

    let exit_code;

    if is_supported() {
        exit_code = provisioner.provision(&configuration);
    } else {
        logger::print_message("Unsupported platform".into(), logger::MessageType::Stderr);
        exit_code = 1;
    }

    // let exit_code = match service.os_type() {
    //     system_services::OSType::Redhat  => provisioner.provision(&configuration),
    //     _                                => {
    //         logger::print_message("Unsupported platform".into(), logger::MessageType::Stderr);
    //         1
    //     }
    // };

    process::exit(exit_code);
}
