use os_type::OSType;
// use system_services::{OSType};


pub struct Platform {
    pub install_command: String,
    pub package_installed_command: String
}

pub fn for_os_type(os_type: OSType) -> Option<Platform> {
    match os_type {
        OSType::Redhat => {
            Some(Platform {
                install_command: "yum install -y".into(),
                package_installed_command: "yum list installed".into()
            })
        },
        _ => None
    }
}
