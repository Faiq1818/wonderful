use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use toml::Table;

pub fn scan_toml(config_path: &PathBuf) {
    assert!(config_path.exists(), "example.toml does not exist");

    let file = fs::read(config_path)
        .expect("could not read example.toml")
        .iter()
        .map(|c| *c as char)
        .collect::<String>();

    println!("{file}");
}
