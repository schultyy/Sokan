extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};
use file;

pub struct Configuration {
    pub packages: Vec<String>,
    pub files: Vec<file::FileResource>,
    pub hostname: String
}

impl Configuration {
    pub fn is_valid(&self) -> bool {
        let mut files_valid = true;
        for file_result in self.files.iter().map(|f| f.is_valid()) {
            if !file_result {
                files_valid = false;
                break;
            }
        }
        files_valid
    }

    pub fn error_messages(&self) -> Vec<String> {
        let mut error_messages = Vec::new();

        let file_errors = self.files.iter()
                            .flat_map(|f| f.error_messages())
                            .collect::<Vec<_>>();
        error_messages.extend(file_errors);

        error_messages
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}

fn convert_yaml_string(yaml_str: &yaml_rust::yaml::Yaml) -> String {
    if let Some(s) = yaml_str.as_str() {
        s.into()
    } else {
        String::new()
    }
}

fn extract_file_resources(file_list: &yaml_rust::yaml::Yaml) -> Vec<file::FileResource> {
    let mut file_resources = Vec::new();
    match file_list.as_vec() {
        Some(lst) => {
            file_resources = lst.iter()
                .map(|e| file::FileResource{
                    content: convert_yaml_string(e.as_hash().unwrap().get(&Yaml::from_str("content")).unwrap()),
                    path: convert_yaml_string(e.as_hash().unwrap().get(&Yaml::from_str("path")).unwrap())
                })
                .collect::<Vec<_>>()
        }
        None => {}
    }
    return file_resources;
}

pub fn extract_hostname(document: &yaml_rust::yaml::Yaml) -> String {
    let doc_hash = document.as_hash().unwrap();
    let keys :Vec<yaml_rust::yaml::Yaml>= doc_hash.keys().cloned().collect();
    match keys.first() {
        Some(host_name) => host_name.as_str().unwrap().into(),
        None            => String::new()
    }
}

pub fn from_yaml(yaml_file: String) -> Configuration {
    let docs = YamlLoader::load_from_str(&yaml_file).unwrap();
    let doc = &docs[0];

    let hostname = extract_hostname(&doc);
    let default_node = doc.as_hash().unwrap().get(&Yaml::from_str(&hostname[..])).unwrap();
    let empty_list = Yaml::Array(Vec::new());
    let package_list = default_node.as_hash().unwrap().get(&Yaml::from_str("packages")).unwrap_or(&empty_list);
    let file_list = default_node.as_hash().unwrap().get(&Yaml::from_str("files")).unwrap_or(&empty_list);
    let mut yaml_packages = Vec::new();

    match package_list.as_vec() {
        Some(lst) => {
            yaml_packages = lst.iter()
                .map(|e| e.as_str().expect("expected string").to_string())
                .collect::<Vec<_>>()
        },
        None => { }
    }

    let file_resources = extract_file_resources(file_list);

    Configuration {
        packages: yaml_packages,
        files: file_resources,
        hostname: hostname
    }
}
