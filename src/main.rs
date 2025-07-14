use std::{env, fs, mem, path::Path};

use toml::Value;

mod my_panic;
use my_panic::panic;

mod cmd_handler;

struct CommandArguments {
    license_name: String,
    directory: String,
    custom_message: String,
}

struct ConfigOptions {
    username: String,
    target_license_filename: String,
    licenses_path: String,
    set_username: bool,
    set_date: bool,
    set_custom_message: bool,
    always_overwrite: bool,
}

const DEFAULT_CONFIG: &str = r#"[User]
Name = "YOUR_NAME"

[Paths]
TargetLicenseFilename = "LICENSE"
LicensesPath = "./licenses"

[Settings]
SetUsername = true
SetDate = true
SetCustomMessage = true
AlwaysOverwrite = true
"#;

fn read_settings() -> ConfigOptions {
    let config_path: String = {
        if cfg!(windows) {
            let home_dir_path = {
                match env::var("USERPROFILE") {
                    Ok(path) => path,
                    Err(_) => {
                        panic("Unable to find your userprofile directory");
                        unreachable!();
                    }
                }
            };
            format!("{}\\gitlicense\\", &home_dir_path)
        } else if cfg!(unix) {
            let home_dir_path = {
                match env::var("HOME") {
                    Ok(path) => path,
                    Err(_) => {
                        panic("Unable to find your home directory");
                        unreachable!();
                    }
                }
            };
            format!("{}/.config/gitlicense/", &home_dir_path)
        } else {
            panic("Your OS is not supported!");
            unreachable!();
        }
    };

    let full_config_path: String = format!("{}/config.toml", config_path);
    if !Path::new(&full_config_path).exists() {
        match fs::create_dir_all(&config_path) {
            Ok(_) => match fs::File::create(&full_config_path) {
                Ok(_) => {
                    fs::write(&full_config_path, &DEFAULT_CONFIG)
                        .expect("Failed to write default configuration in your configuration file");
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
                    .expect("Failed to write default configuration in your configuration file");
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
        username: value["User"]["Name"].to_string().replace('"',""),
        target_license_filename: value["Paths"]["TargetLicenseFilename"].to_string().replace('"',""),
        licenses_path: format!("{}",value["Paths"]["LicensesPath"]).replace('"',"").replace('.',&config_path),
        set_username: value["Settings"]["SetUsername"].as_bool().expect("Error while reading SetUsername setting in your configuration file,cannot estabilish true/false option"),
        set_date: value["Settings"]["SetDate"].as_bool().expect("Error while reading SetDate setting in your configuration file,cannot estabilish true/false option"),
        set_custom_message: value["Settings"]["SetCustomMessage"].as_bool().expect("Error while reading SetCustomMessage setting in your configuration file,cannot estabilish true/false option"),
        always_overwrite:value["Settings"]["AlwaysOverwrite"].as_bool().expect("Error while reading AlwaysUpdate setting in your configuration file,cannot estabilish true/false option"),
    };
    return config_options;
}

fn correct_args(mut cmd: Vec<String>) -> Vec<String> {
    if cmd.len() < 2 {
        cmd.push("--help".to_string());
    }
    if cmd.len() < 3 {
        cmd.push(".".to_string());
    }
    if cmd.len() < 4 {
        cmd.push("".to_string());
    }
    return cmd;
}

fn main() {
    let mut cmd: Vec<String> = env::args().collect();
    cmd = correct_args(cmd);
    if cmd.len() < 4 {
        panic("Not enough arguments provided!")
    }

    let args = CommandArguments {
        license_name: mem::take(&mut cmd[1]),
        directory: mem::take(&mut cmd[2]),
        custom_message: mem::take(&mut cmd[3]),
    };
    let config_options = read_settings();
    cmd_handler::handle(args, config_options);
}
