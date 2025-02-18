use crate::{panic, CommandArguments, ConfigOptions};
use chrono::Datelike;
use std::{fs, path::Path, process};

pub fn handle(args: CommandArguments, config: ConfigOptions) {
    match args.license_name.as_str() {
        "--help" => show_help(),
        _ => {
            let target_file = format!("{}/{}", args.directory, config.target_license_filename);
            if !Path::new(&target_file).exists()
                || Path::new(&target_file).exists() && config.always_update == true
            {
                let license_path = format!("{}/{}", config.licenses_path, args.license_name);

                let mut license_content =
                    fs::read_to_string(license_path).expect("Failed to read license file");
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
                        "File was sucessfully created in {} directory!",
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
                println!("File {} already exists in this directory,\nif you want to enable updating license file set AlwaysUpdate = true in your configuration file",config.target_license_filename);
                process::exit(0)
            }
        }
    }
}
fn show_help() {
    println!(
        "Usage: gitlicense [license] [directory (current by default)] [custom_message (optional)]"
    );
    process::exit(0);
}
