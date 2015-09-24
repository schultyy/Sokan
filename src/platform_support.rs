use system_services;
use system_services::SystemInterface;

pub fn is_supported() -> bool {
    let service = system_services::SystemServices;
    match service.os_type() {
        system_services::OSType::Redhat => true,
        _ => false
    }
}
