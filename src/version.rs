use std::{fs::{create_dir, remove_dir_all}, path::PathBuf, io::{self, BufRead}};
use super::config::Config;
use super::config;

// create:  creates a new version (that is the .kadot file and directory)
// version: name of the version to create
//
// return:  true if the version was created correctly, false otherwise
pub fn create(version: &String) -> bool {
    // the version is created in the current directory
    let current_path = std::fs::canonicalize(PathBuf::from(".")).unwrap();
    println!("Creating version: {version} in path {}", current_path.to_str().unwrap());
    let mut config: Config;
    let config_string = std::fs::read_to_string(current_path.join(".kadot"));

    // if the current directory is a version already, it asks if you want to create
    // a sub-version
    if let Ok(config_string) = config_string {
        let mut buffer = String::with_capacity(2048);
        config = serde_json::from_str(&config_string).expect(".kadot is not in a valid format");
        println!("Found current version: {}", config::get_name(&config));
        println!("Do you wish to create a sub-version (or quit)? [y,N]");
        buffer.clear();
        assert!(io::stdin().lock().read_line(&mut buffer).is_ok());

        // creates a sub-version or exits
        match buffer.as_str() {
            "Y\n" | "y\n" => {
                if config::exist_version(&config, version) {
                    println!("Sub-version with the same name already exists");
                    return false;
                }
                let version_path = current_path.join(version);
                println!("scendo in {}", version_path.to_str().unwrap());
                /* creates the directory of the sub-version */
                if !version_path.is_dir() {
                    if let Err(_) = create_dir(&version_path) {
                        println!("Couldn't create sub-folder");
                        return false;
                    }
                }

                if !version_path.join(".kadot").is_file() {
                    // creates the config of the new version
                    let new_config = super::config::create_config(version,  Some(&config));
                    let json = serde_json::to_string_pretty(&new_config).unwrap();
                    // creates the .kadot file of the sub-version
                    std::fs::write(version_path.join(".kadot"), json).expect("Couldn't write to file");
                }

                // updates the config of the parent version
                config::add_version(&mut config, version);
                let json = serde_json::to_string_pretty(&config).unwrap();
                if let Err(_) = std::fs::write(current_path.join(".kadot"), json) {
                    println!("Couldn't create .kadot file");
                    assert!(remove_dir_all(version_path).is_ok());
                }
                return false;
            },
            _             => return false,
        }
    }
    // if we aren't in a sub-version, it creates a new config and put it into the
    // current directory
    else {
        config = super::config::create_config(version, None);
        let json = serde_json::to_string_pretty(&config).unwrap();
        println!("Current path {}", current_path.to_str().unwrap());
        std::fs::write(current_path.join(".kadot"), json).expect("Couldn't write .kadot");

        return true;
    }
}

// install:      installs one of the sub-version
// version:      name of the version to install. If none, it asks the user
// current_path: path that contains the sub-version, needed in case of recursion
pub fn install(version: &Option<String>, current_path: PathBuf) {
    // we get the config of the whole version
    let config_string = match std::fs::read_to_string(current_path.join(".kadot")) {
        Err(_) => return,
        Ok(config_string) => config_string
    };
    let config: Config = serde_json::from_str(&config_string).expect(".kadot is not in a valid format");

    // if a version is Some, we check that it exists, else it asks the user
    let version_name = match version {
        Some(version_name) => {
            if !super::config::exist_version(&config, &version_name) {
                println!("Version not found");
                return;
            }
            version_name
        }
        None => {
            let versions = config::get_versions(&config);
            println!("Choose one of the available versions:");
            for (i, subver) in versions.into_iter().enumerate() {
                println!("{}: {}", i+1, subver);
            }
            let input = super::io::prompt_user();
            let input: usize = input.trim().parse().expect("Wanted a number");
            match versions.get(input-1) {
                Some(version_name) => version_name,
                None => return,
            }
        }
    };

    // we enter the subversion dir and get the .kadot. If it has subversions,
    // we asks which one to install recursively, else it gets installed
    let version_path = current_path.join(version_name);
    if version_path.is_dir() {
        match std::fs::read_to_string(version_path.join(".kadot")) {
            Ok(config_string) => {
                let config: Config = serde_json::from_str(&config_string).expect(".kadot malformed");
                let versions = config::get_versions(&config);
                if versions.is_empty() {
                    config::install_config(config, current_path);
                }
                else {
                    install(&None, version_path);
                }
            },
            Err(_) => return,
        }
    }
}
