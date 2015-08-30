use system_services;

pub enum PlatformType {
    debian,
    redhat,
    unknown
}

pub fn platform() -> PlatformType {
    if system_services::file_exists(&"/etc/redhat-release".to_string()) ||
        system_services::file_exists(&"/etc/centos-release".to_string()){
            PlatformType::redhat
    } else {
        PlatformType::unknown
    }
}
