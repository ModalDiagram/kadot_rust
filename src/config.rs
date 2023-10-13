use std::io::{self, BufRead};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Config {
    name: String,
    target: String,
    info: String,
    ignore: Vec<String>,
    versions: Vec<String>,
}

pub fn get_name(config: &Config) -> &str {
    &config.name
}

pub fn get_versions(config: &Config) -> &Vec<String> {
    &config.versions
}

pub fn add_version(config: &mut Config, version: &str) {
    config.versions.push(version.to_string())
}

pub fn exist_version(config: &Config, version: &str) -> bool {
    config.versions.iter().find(|&ver| ver == version).is_some()
}

// create_config: creates the object Config of a new version
// version:       name of the version to be created
//
// return:        the object Config with the info of the new version
pub fn create_config(version: &String, old_config: Option<&Config>) -> Config {
    let mut buffer: String = String::with_capacity(2048);
    let mut reader = io::stdin().lock();
    // Initialize the object
    let mut new_config: Config = Config { name: String::from(""), target: String::from(""), info: String::from(""), ignore: vec![], versions: vec![] };

    new_config.name = version.clone();

    // asks for the target directory of the new version
    // if we are in a sub-version, it asks if the target directory is the same
    // as the parent target directory, else it asks for the new one
    if old_config.is_some(){
        println!("Does this sub-version have the same target? ({}) [y,n]", old_config.unwrap().target);
        assert!(reader.read_line(&mut buffer).is_ok());
        match buffer.as_str() {
            "y\n" | "Y\n" => {
                buffer.clear();
                new_config.target = old_config.unwrap().target.clone();
            },
            _             => {
                buffer.clear();
                println!("Choose the new target directory:");
                assert!(reader.read_line(&mut buffer).is_ok());
                new_config.target = String::from(buffer.clone().trim());
                buffer.clear();
            }
        }
    }
    // if we are not in a sub-version it asks for the target directory
    else {
        println!("Choose the default target directory:");
        assert!(reader.read_line(&mut buffer).is_ok());
        new_config.target = String::from(buffer.clone().trim());
        buffer.clear();
    }

    // asks for any info of the new version
    println!("Choose the info:");
    assert!(reader.read_line(&mut buffer).is_ok());
    new_config.info = String::from(buffer.clone().trim());
    buffer.clear();

    return new_config;
}
