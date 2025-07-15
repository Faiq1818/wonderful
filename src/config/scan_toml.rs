use std::process::Command;
use toml;

pub type TomlFile = std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>;

pub fn scan_toml(config_path: &std::path::PathBuf, selected_item: &str) {
    assert!(config_path.exists(), "example.toml does not exist");

    let file_str = std::fs::read_to_string(config_path).unwrap();
    let parsed: TomlFile = toml::from_str(&file_str).expect("invalid toml structure");

    for (section, kvs) in &parsed {
        println!("{}", section);
        if selected_item == section {
            for (key, val) in kvs {
                println!("{}",key);
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
    let parsed: TomlFile = toml::from_str(&file_str).expect("invalid toml structure");

    //let mut items = vec![];
    let items = parsed.keys().cloned().collect::<Vec<_>>();
    for kvs in parsed.values() {
        //items.push(kvs.to_string());
        for _key in kvs.keys() {
            //items.push(key.to_string());
        }
    }
    items
}
