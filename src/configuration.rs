extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};
use command;

pub struct Configuration {
    pub commands: Vec<command::Command>,
    pub packages: Vec<String>,
    pub install_command: Option<String>
}

pub fn from_yaml(yaml_file: String) -> Configuration {
    let docs = YamlLoader::load_from_str(&yaml_file).unwrap();
    let doc = &docs[0];

    let default_node = doc.as_hash().unwrap().get(&Yaml::from_str("default")).unwrap();
    let empty_list = Yaml::Array(Vec::new());
    let command_list = default_node.as_hash().unwrap().get(&Yaml::from_str("commands")).unwrap_or(&empty_list);
    let package_list = default_node.as_hash().unwrap().get(&Yaml::from_str("packages")).unwrap_or(&empty_list);
    let yaml_install_command = default_node.as_hash().unwrap().get(&Yaml::from_str("package_install_cmd"));
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

    let mut install_cmd = None;

    match yaml_install_command {
        Some(cmd) => {
            let s = cmd.as_str().unwrap();
            install_cmd = Some(s.to_string())
        },
        None => {}
    }

    Configuration {
        commands: yaml_commands,
        packages: yaml_packages,
        install_command: install_cmd
    }
}
