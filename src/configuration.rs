extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};
use command;

pub struct Configuration {
    pub commands: Vec<command::Command>
}

pub fn from_yaml(yaml_file: String) -> Configuration {
    let docs = YamlLoader::load_from_str(&yaml_file).unwrap();
    let doc = &docs[0];

    let key = Yaml::from_str("default");
    let default_command_list = doc.as_hash().unwrap().get(&key).unwrap();
    let yaml_commands = default_command_list.as_vec().unwrap();
    Configuration {
        commands: yaml_commands.iter()
                 .map(|e| command::parse(e.as_str().expect("expected string").to_string()))
                 .collect::<Vec<_>>()
    }
}
