extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};

pub fn from_yaml(yaml_file: String) -> Vec<String> {
    let docs = YamlLoader::load_from_str(&yaml_file).unwrap();
    let doc = &docs[0];

    let key = Yaml::from_str("default");
    let default_command_list = doc.as_hash().unwrap().get(&key).unwrap();
    let yaml_commands = default_command_list.as_vec().unwrap();

    let mut result_commands = Vec::new();
    result_commands.extend(
      yaml_commands.iter().map(|e| element.as_str().unwrap().to_string()
    )

    result_commands
}
