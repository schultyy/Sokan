use system_services;
use os_type;
use system_services::SystemInterface;

pub fn is_supported() -> bool {
    let service = system_services::SystemServices;
    match service.os_type() {
        os_type::OSType::Redhat => true,
        _ => false
    }
}
