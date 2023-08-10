use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, io::Error as IOError, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    main: Option<ConfigTomlMain>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlMain {
    widgets: Option<String>,
    separator: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub widgets: Vec<String>,
    pub separator: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            widgets: vec![
                "nameu".to_string(),
                "os_vewsiown".to_string(),
                "howst_name".to_string(),
                "kewnel_vewsion".to_string(),
                "cpuwu".to_string(),
                "totaw_memowy".to_string(),
                "used_memowy".to_string(),
            ],
            separator: ">".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut user_config_filepath: String = "./kawaii.toml".to_string();
        if let Some(base_dirs) = BaseDirs::new() {
            let mut kawaii_config_path = PathBuf::from(base_dirs.config_dir());
            kawaii_config_path.push("kawaiifetch/kawaii.toml");

            if kawaii_config_path.exists() {
                match kawaii_config_path.into_os_string().into_string() {
                    #![allow(clippy::single_match)]
                    Ok(config_path) => {
                        user_config_filepath = config_path;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        };

        // prioritize user config
        let config_filepaths: [&str; 2] = [user_config_filepath.as_str(), "./default_config.toml"];

        let mut content: String = "".to_string();

        for filepath in config_filepaths {
            let config_path = PathBuf::from(filepath);
            let config_file_contents: Result<String, IOError> = read_to_string(config_path);

            match config_file_contents {
                Ok(result) => {
                    content = result;
                    break;
                }
                Err(e) => {
                    println!("Error: {}; {}", e, filepath);
                }
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to parse config file.");
            ConfigToml { main: None }
        });

        let (widgets, separator): (Vec<String>, String) = match config_toml.main {
            Some(config_main) => {
                let main_widgets: String = config_main.widgets.unwrap_or_else(|| {
                    println!("No widgets field specified in main.\nUsing defaults");
                    Config::default().widgets.join(" ")
                });

                let separator = config_main.separator.unwrap_or_else(|| {
                    println!("No separator field specified in main.\nUsing default");
                    Config::default().separator
                });

                (
                    main_widgets
                        .split_whitespace()
                        .map(|w| w.to_string())
                        .collect(),
                    separator,
                )
            }
            None => {
                println!("Missing table main.");
                (Config::default().widgets, Config::default().separator)
            }
        };

        Config { widgets, separator }
    }
}
