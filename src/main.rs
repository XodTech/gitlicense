use std::{env, fs, mem, path::Path};

use toml::Value;

mod my_panic;
use my_panic::panic;

mod cmd_handler;

struct CommandArguments {
    license_name: String,
    directory: String,
}

struct ConfigOptions {
    username: String,
    liceenses_path: String,
    set_date: bool,
    always_update: bool,
}

const DEFAULT_CONFIG: &str = r#"
[User]
Name = "YOUR_NAME"
[Settings]
LicensesPath = "./licenses"
SetDate = true
AlwaysUpdate = true
"#;

fn read_settings() -> ConfigOptions {
    let homedir_path = {
        if cfg!(windows) {
            env::var("USERPROFILE").ok()
        } else if cfg!(unix) {
            env::var("HOME").ok()
        } else {
            panic("This system is unsuported");
            None
        }
    };
    let config_path: String = {
        match homedir_path {
            Some(path) => format!("{}/.config/gitlicense", path),
            None => {
                panic("Unable to find your home directory");
                "".to_string()
            }
        }
    };
    let full_config_path: String = format!("{}/config.toml", config_path);
    if !Path::new(&full_config_path).exists() {
        match fs::create_dir_all(&config_path) {
            Ok(_) => match fs::File::create(&full_config_path) {
                Ok(_) => {
                    fs::write(&full_config_path, &DEFAULT_CONFIG).expect(
                        "Failed to write default file structure in your configuration file",
                    );
                    println!(
                        "Configuration file was succesfully created with a default configuration in: {}",
                        &full_config_path
                    )
                }
                Err(e) => panic(
                    format!(
                        "Error while creating configuration file,error message: {}",
                        e
                    )
                    .as_str(),
                ),
            },
            Err(e) => panic(
                format!(
                    "Error while creating configuration directory,error message: {}",
                    e
                )
                .as_str(),
            ),
        }
    } else if Path::new(&config_path).exists() && !Path::new(&full_config_path).exists() {
        match fs::File::create(&full_config_path) {
            Ok(_) => {
                fs::write(&full_config_path, &DEFAULT_CONFIG)
                    .expect("Failed to write default file structure in your configuration file");
                println!(
                    "Configuration file was succesfully created with a default configuration in: {}",
                    &full_config_path
                )
            }
            Err(e) => panic(
                format!(
                    "Error while creating configuration file,error message: {}",
                    e
                )
                .as_str(),
            ),
        }
    }
    // TODO:Automatically add commented default structure for toml configuration file
    let toml_string: String =
        fs::read_to_string(&full_config_path).expect("Failed to read config file");
    let value = toml_string
        .parse::<Value>()
        .expect("Error while parsing config file");
    let config_options = ConfigOptions {
        username: value["User"]["Name"].to_string(),
        liceenses_path: value["Settings"]["LicensesPath"].to_string(),
        set_date: value["Settings"]["SetDate"].as_bool().expect("Error while reading SetDate setting in your configuration file,cannot estabilish true/false option"),
        always_update:value["Settings"]["AlwaysUpdate"].as_bool().expect("Error while reading AlwaysUpdate setting in your configuration file,cannot estabilish true/false option"),
    };
    return config_options;
}

fn correct_args(mut cmd: Vec<String>) -> Vec<String> {
    if cmd.len() < 1 {
        cmd.push("help".to_string())
    }
    if cmd.len() < 2 {
        cmd.push(".".to_string())
    }
    return cmd;
}

fn main() {
    let mut cmd: Vec<String> = env::args().collect();
    cmd = correct_args(cmd);
    let args = CommandArguments {
        license_name: mem::take(&mut cmd[1]),
        directory: mem::take(&mut cmd[2]),
    };
    let config_options = read_settings();
    cmd_handler::handle(args, config_options);
}
