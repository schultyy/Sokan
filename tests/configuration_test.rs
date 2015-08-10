#[path="../src/configuration.rs"]
mod configuration;

#[test]
fn returns_list_of_commands() {
    let yaml_file = "
    default:
      - sudo apt-get update
    ";
    let commands = configuration::from_yaml(yaml_file.to_string());
    assert_eq!(commands, ["sudo apt-get update"]);
}
