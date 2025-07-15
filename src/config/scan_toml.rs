use std::process::Command;
use toml;

pub type TomlFile = std::collections::HashMap<String, std::collections::HashMap<String, String>>;
pub type TomlFile2 = std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>;

pub fn scan_toml(config_path: &std::path::PathBuf, selected_item: &str) {
    assert!(config_path.exists(), "example.toml does not exist");

    let file_str = std::fs::read_to_string(config_path).unwrap();
    let parsed: TomlFile = toml::from_str(&file_str).expect("invalid toml structure");

    for kvs in parsed.values() {
        for (key, val) in kvs {
            if selected_item == key {
                let args: Vec<&str> = val.split_whitespace().collect();
                if !args.is_empty() {
                    let _ = Command::new(args[0]).args(&args[1..]).spawn();
                }
            }
        }
    }
}

pub fn get_items_name(config_path: &std::path::PathBuf) -> Vec<String> {
    let file_str = std::fs::read_to_string(config_path).unwrap();
    let parsed: TomlFile2 = toml::from_str(&file_str).expect("invalid toml structure");

    let mut items = vec![];
    for kvs in parsed.values() {
        for key in kvs.keys() {
            let args: Vec<&str> = key.split_whitespace().collect();

            for s in args {
                items.push(s.to_string());
            }
        }
    }
    items
}
