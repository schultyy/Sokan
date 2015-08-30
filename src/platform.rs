use system_services::SystemServices;

pub enum PlatformType {
    debian,
    redhat,
    unknown
}

pub fn platform() -> PlatformType {
    let service = SystemServices;
    if service.file_exists(&"/etc/redhat-release".to_string()) ||
        service.file_exists(&"/etc/centos-release".to_string()){
            PlatformType::redhat
    } else {
        PlatformType::unknown
    }
}
