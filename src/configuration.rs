extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};
use command;

pub struct Configuration {
    pub commands: Vec<command::Command>,
    pub packages: Vec<String>
}

pub fn from_yaml(yaml_file: String) -> Configuration {
    let docs = YamlLoader::load_from_str(&yaml_file).unwrap();
    let doc = &docs[0];

    let default_node = doc.as_hash().unwrap().get(&Yaml::from_str("default")).unwrap();
    let default_value = Yaml::Array(Vec::new());
    let command_list = default_node.as_hash().unwrap().get(&Yaml::from_str("commands")).unwrap_or(&default_value);
    let package_list = default_node.as_hash().unwrap().get(&Yaml::from_str("packages")).unwrap_or(&default_value);
    let mut yaml_commands = Vec::new();
    let mut yaml_packages = Vec::new();

    match command_list.as_vec() {
        Some(lst) => {
            yaml_commands = lst.iter()
                 .map(|e| command::parse(e.as_str().expect("expected string").to_string()))
                 .collect::<Vec<_>>()
        },
        None => {}
    }

    match package_list.as_vec() {
        Some(lst) => {
            yaml_packages = lst.iter()
                .map(|e| e.as_str().expect("expected string").to_string())
                .collect::<Vec<_>>()
        },
        None => { }
    }

    Configuration {
        commands: yaml_commands,
        packages: yaml_packages
    }
}
