use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
//use toml::Table;

#[derive(Deserialize, Debug)]
struct TomlFile {
    items: Items,
}

#[derive(Deserialize, Debug)]
struct Items {
    first: String,
    second: String,
}

pub fn scan_toml(config_path: &PathBuf) {
    assert!(config_path.exists(), "example.toml does not exist");

    let file = fs::read(config_path)
        .expect("could not read example.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();

    println!("{file}");
    
    let file_str = std::fs::read_to_string(config_path).unwrap();
    let tomlfile: TomlFile = toml::from_str(&file_str).unwrap();

    println!("{}", tomlfile.items.first);
    println!("{}", tomlfile.items.second);
}
