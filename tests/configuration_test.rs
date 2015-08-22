#[path="../src/configuration.rs"]
mod configuration;
#[path="../src/command.rs"]
mod command;

#[test]
fn returns_configuration_with_list_of_commands() {
    let yaml_file = "
    default:
      - sudo apt-get update
    ";
    let configuration = configuration::from_yaml(yaml_file.to_string());
    let expected = command::Command {
        sudo: true,
        command: "apt-get".to_string(),
        args: vec!["update".to_string()]
    };
    let actual = configuration.commands.first().unwrap();
    assert_eq!(expected.sudo, actual.sudo);
    assert_eq!(expected.command, actual.command);
    assert_eq!(expected.args, actual.args);
}
