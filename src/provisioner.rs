use configuration;
use logger;
use file;
use system_services::{SystemServices, SystemInterface};

pub struct Provisioner{
    system_service: SystemServices
}

impl Provisioner {
    pub fn new(system_service: SystemServices) -> Provisioner {
        Provisioner {
            system_service: system_service
        }
    }

    pub fn provision(&self, configuration: &configuration::Configuration) -> i32 {
        let mut packages = Vec::new();

        if !configuration.is_valid() {
            for error_message in configuration.error_messages() {
                logger::print_message(error_message, logger::MessageType::Stderr);
            }
            return 1;
        }


        packages.extend(configuration.packages.iter().cloned());

        packages.reverse();

        let mut exit_codes = Vec::new();

        if self.manage_hostname(&configuration) {
            exit_codes.push(0);
        } else {
            exit_codes.push(1);
        }

        let package_exit_codes = self.install_packages(&packages);
        exit_codes.extend(&package_exit_codes[..]);

        let results = self.handle_file_resources(&configuration.files);
        exit_codes.extend(&results[..]);

        exit_codes.sort();

        match exit_codes.last() {
            Some(&0) => return 0,
            None     => return 0,
            _        => return 1
        }
    }

    fn manage_hostname(&self, configuration: &configuration::Configuration) -> bool {
        if configuration.hostname == "default" {
            return true
        }

        let current_hostname = match self.system_service.get_hostname() {
            Some(hn)    => hn,
            None        => String::new()
        };

        if current_hostname == configuration.hostname.to_string() {
            return true
        } else {
            match self.system_service.set_hostname(&configuration.hostname) {
                true => {
                    logger::print_message(format!("==> Set hostname to {}", configuration.hostname), logger::MessageType::Stdout);
                    true
                },
                false => {
                    logger::print_message(format!("==> Failed to set hostname to {}", configuration.hostname), logger::MessageType::Stderr);
                    false
                }
            }
        }
    }

    fn install_packages(&self, package_list: &Vec<String>) -> Vec<i32> {
        let mut exit_codes = Vec::new();
        let mut packages = package_list.clone();

        while let Some(package) = packages.pop() {
            if self.system_service.is_package_installed(&package) {
                continue;
            }
            let shellout = self.system_service.install_package(&package);
            logger::print_shellout::<String>(&package, &shellout);
            let exit_status = shellout.status.clone();
            if exit_status.success() == false {
                exit_codes.push(exit_status.code().unwrap());
            }
        }
        exit_codes
    }

    fn file_is_equal_to(&self, file_resource: &file::FileResource) -> bool {
        //TODO better error handling here
        let other_file = file::FileResource::create_from_path(&file_resource.path).unwrap();
        other_file.hash() == file_resource.hash()
    }

    fn handle_file_resources(&self, resources: &Vec<file::FileResource>) -> Vec<i32> {
        let mut results = Vec::new();
        for resource in resources {
            if self.system_service.file_exists(&resource.path) && self.file_is_equal_to(&resource) {
                results.push(0);
                continue;
            }
            let result = resource.write_file();
            match result {
                Ok(_) =>  {
                    let msg = format!("==> Wrote file {}", resource.path);
                    logger::print_message(msg, logger::MessageType::Stdout);
                    results.push(0);
                },
                Err(err) => {
                    let msg = format!("==> Error while writing file {}", resource.path);
                    logger::print_message(msg, logger::MessageType::Stderr);
                    logger::print_message(format!("==> {}", err), logger::MessageType::Stderr);
                    results.push(1);
                }
            }
        }
        results
    }
}
