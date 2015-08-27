#[path="../src/configuration.rs"]
mod configuration;

#[path="../src/file.rs"]
mod file;

#[test]
fn test_returns_configuration_with_empty_list_of_packages() {
    let yaml_file = "
    default:
        packages:
    ";
    let configuration = configuration::from_yaml(yaml_file.to_string());
    assert_eq!(configuration.packages.is_empty(), true);
}

#[test]
fn test_returns_configuration_with_list_of_packages() {
    let yaml_file = "
    default:
      packages:
        - vim
        - git
        - build-essential
    ";

    let configuration = configuration::from_yaml(yaml_file.to_string());
    assert_eq!(configuration.packages, vec!["vim", "git", "build-essential"]);
}

#[test]
fn test_returns_configuration_with_install_command() {
    let yaml_file = "
    default:
      packages:
        - vim
        - git
      package_install_cmd: apt-get install -y
      ";
    let configuration = configuration::from_yaml(yaml_file.to_string());
    assert_eq!(configuration.install_command, Some("apt-get install -y".to_string()));
}

#[test]
fn test_returns_configuration_without_install_command() {
    let yaml_file = "
    default:
      packages:
        - vim
        - git
  ";
  let configuration = configuration::from_yaml(yaml_file.to_string());
  assert_eq!(configuration.install_command, None);

}

#[test]
fn test_returns_configuration_with_a_file_resource() {
    let yaml_file = "
    default:
      files:
        -
          path: '/home/Jane/hello.txt'
          content: 'Hi from John'
    ";

    let configuration = configuration::from_yaml(yaml_file.to_string());
    assert_eq!(configuration.files.len(), 1);
}

#[test]
fn test_returns_configuration_with_a_file_resource_has_all_properties() {
    let yaml_file = "
    default:
      files:
        -
          path: '/home/Jane/hello.txt'
          content: 'Hi from John'
    ";

    let configuration = configuration::from_yaml(yaml_file.to_string());
    let file = configuration.files.first().unwrap();
    assert_eq!(file.path, "/home/Jane/hello.txt");
    assert_eq!(file.content, "Hi from John");
}

#[test]
fn test_valid_configuration() {
    let configuration = configuration::Configuration {
        packages: vec![],
        files: vec![file::FileResource{
            path: "/home/john/hello.txt".to_string(),
            content: "hello".to_string()
        }],
        install_command: Some("yum install -y".to_string())
    };

    assert_eq!(configuration.is_valid(), true);
}

#[test]
fn test_invalid_configuration() {
    let configuration = configuration::Configuration {
        packages: vec![],
        files: vec![file::FileResource{
            path: "/home/john/hello.txt".to_string(),
            content: "hello".to_string()
        }],
        install_command: None
    };

    assert_eq!(configuration.is_valid(), false);
}

#[test]
fn test_configuration_valid_with_empty_file_list() {
    let configuration = configuration::Configuration {
        packages: vec![],
        files: vec![],
        install_command: Some("yum install -y".to_string())
    };

    assert_eq!(configuration.is_valid(), true);
}

#[test]
fn test_return_no_error_messages_if_configuration_is_valid() {
    let configuration = configuration::Configuration {
        packages: vec![],
        files: vec![],
        install_command: Some("yum install -y".to_string())
    };

    assert_eq!(configuration.error_messages().is_empty(), true);
}
