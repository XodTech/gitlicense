use crate::{panic, CommandArguments, ConfigOptions};
use chrono::Datelike;
use std::{fs, path::Path, process};
use toml::Value;

pub fn handle(args: CommandArguments, config: ConfigOptions) {
    match args.license_name.as_str() {
        "--help" => show_help(),
        _ => {
            let target_file: String =
                format!("{}/{}", args.directory, config.target_license_filename);
            if !Path::new(&target_file).exists()
                || Path::new(&target_file).exists() && config.always_overwrite == true
            {
                let license_path: String =
                    format!("{}/{}", config.licenses_path, args.license_name);
                let license_extention: &str = match Path::new(&license_path).extension() {
                    Some(ext) => ext.to_str().unwrap_or("none"),
                    None => "none",
                };
                if license_extention != "toml" {
                    // When is a simple (one file license) which has not toml extention
                    let mut license_content: String =
                        fs::read_to_string(&license_path).expect("Failed to read license file");
                    if config.set_username == true {
                        license_content = license_content.replace("[USERNAME]", &config.username);
                    }
                    if config.set_date == true {
                        let current_date = chrono::Utc::now();
                        license_content =
                            license_content.replace("[DATE]", &current_date.year().to_string());
                    }
                    if config.set_custom_message == true {
                        license_content =
                            license_content.replace("[CUSTOM_MESSAGE]", &args.custom_message);
                    }
                    match fs::write(&target_file, &license_content) {
                        Ok(_) => println!(
                            "License file was sucessfully written in {} directory!",
                            args.directory.replace('.', "this")
                        ),
                        Err(e) => panic(
                            format!(
                                "Error while writting to {}\nError:{}",
                                config.target_license_filename, e
                            )
                            .as_str(),
                        ),
                    }
                } else {
                    // When is a complex license(several files)
                    let toml_string: String =
                        fs::read_to_string(&license_path).expect("Failed to read config file");
                    let value = toml_string
                        .parse::<Value>()
                        .expect("Error while parsing config file");

                    // Writting license first
                    let mut license_content: String =
                        value["LICENSE"]["Content"].to_string().replace('"', "");
                    if config.set_username == true {
                        license_content = license_content.replace("[USERNAME]", &config.username);
                    }
                    if config.set_date == true {
                        let current_date = chrono::Utc::now();
                        license_content =
                            license_content.replace("[DATE]", &current_date.year().to_string());
                    }
                    if config.set_custom_message == true {
                        license_content =
                            license_content.replace("[CUSTOM_MESSAGE]", &args.custom_message);
                    }
                    match fs::write(&target_file, &license_content) {
                        Ok(_) => println!(
                            "License file was sucessfully written in {} directory!",
                            args.directory.replace('.', "this")
                        ),
                        Err(e) => panic(
                            format!(
                                "Error while writting to {}\nError:{}",
                                config.target_license_filename, e
                            )
                            .as_str(),
                        ),
                    }
                    // Writting full license

                    match fs::write(
                        value["FULL_LICENSE"]["Filename"]
                            .to_string()
                            .replace('"', ""),
                        value["FULL_LICENSE"]["Content"]
                            .to_string()
                            .replace('"', ""),
                    ) {
                        Ok(_) => println!(
                            "Full license was sucessfully written in {} directory!",
                            args.directory.replace('.', "this")
                        ),
                        Err(e) => panic(
                            format!(
                                "Error while writting full license to {}\nError:{}",
                                value["FULL_LICENSE"]["Filename"], e
                            )
                            .as_str(),
                        ),
                    }
                }
            } else {
                println!("File {} already exists in this directory,\nif you want to enable updating license file set AlwaysUpdate = true in your configuration file",config.target_license_filename);
                process::exit(0)
            }
        }
    }
}
fn show_help() {
    let version:&str = env!("CARGO_PKG_VERSION");
    println!(
        r#"

gitlicense version {}
Usage: gitlicense [license] [directory] [custom_message]

Add a license to your Git repository.

Arguments:
* license: The type of license to add (required)
* directory: The directory to add the license to (default: current directory)
* custom_message: A custom message to include in the license (optional)

Examples:
  gitlicense MIT ./myproject
  gitlicense Apache-2.0 ./myproject "Licensed for personal and commercial use"
  gitlicense GPL-3.0.toml .
"#,version
    );
    process::exit(0);
}
