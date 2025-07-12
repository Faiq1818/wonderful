use std::process::Command;
use toml;

pub type TomlFile = std::collections::HashMap<String, std::collections::HashMap<String, String>>;

pub fn scan_toml(config_path: &std::path::PathBuf, selected_item: &str) {
    assert!(config_path.exists(), "example.toml does not exist");

    let file_str = std::fs::read_to_string(config_path).unwrap();
    let parsed: TomlFile = toml::from_str(&file_str).expect("invalid toml structure");

    for (_section, kvs) in &parsed {
        for (key, val) in kvs {
            //println!("{} = {}", key, val);
            if selected_item == key {
                //let _ = Command::new("kitty").arg("nvim").spawn();
                let args: Vec<&str> = val.split_whitespace().collect();
                if !args.is_empty() {
                    let _ = Command::new(args[0]).args(&args[1..]).spawn();
                }
            }
        }
    }
}
