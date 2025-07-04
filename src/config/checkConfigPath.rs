use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_dotconfig_path(folderName: &str, fileName: &str) -> PathBuf {
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    config_dir.join(app_name).join(file_name)
}

pub fn ensure_config_exists(config_path: &PathBuf) {
    if !config_path.exists() {
        let parent_dir = config_path.parent().unwrap();
        fs::create_dir_all(parent_dir).expect("Failed to create config directory");

        let default_config = r#"
[settings]
overwrite = true
backup = false
"#;
        let mut file = fs::File::create(config_path).expect("Failed to create config file");
        file.write_all(default_config.as_bytes())
            .expect("Failed to write to config file");

        println!("Created default config at {}", config_path.display());
    }
}
