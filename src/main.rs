use std::{fs::{File, create_dir, remove_dir_all}, path::PathBuf, io::{self, BufRead, Read, Write}};
use serde::{Deserialize, Serialize};
use clap::{Parser,Subcommand};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    name: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Config {
    name: String,
    target: String,
    info: String,
    ignore: Vec<String>,
    versions: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        version: String,
    },
    Install {
        version: String,
    },
}

// create_config: creates the object Config of a new version
// version:       name of the version to be created
//
// return:        the object Config with the info of the new version
fn create_config(version: &String, old_config: Option<&Config>) -> Config {
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

// create:  creates a new version (that is the .kadot file and directory)
// version: name of the version to create
//
// return:  true if the version was created correctly, false otherwise
fn create(version: &String) -> bool {
    // the version is created in the current directory
    let mut path = std::fs::canonicalize(PathBuf::from(".")).unwrap();
    println!("Creating version: {version} in path {}", path.to_str().unwrap());
    let config_file = File::options().write(true).read(true).open(path.as_path().join(".kadot"));
    let mut config: Config;

    // if the current directory is a version already, it asks if you want to create
    // a sub-version
    if config_file.is_ok() {
        let mut config_file = config_file.unwrap();
        let mut buffer = String::with_capacity(2048);
        assert!(config_file.read_to_string(&mut buffer).is_ok());
        config = serde_json::from_str(buffer.as_str()).unwrap();
        println!("Found current version: {}", config.name);
        println!("Do you wish to create a sub-version? [y,n]");
        buffer.clear();
        assert!(io::stdin().lock().read_line(&mut buffer).is_ok());

        // creates a sub-version or exits
        match buffer.as_str() {
            "Y\n" | "y\n" => {
                PathBuf::push(&mut path, version);
                println!("scendo in {}", path.to_str().unwrap());
                /* creates the directory of the sub-version */
                if !(create_dir(&path).is_ok()) {
                    println!("Couldn't create sub-folder");
                    return false;
                }

                // creates the config of the new version
                let new_config = create_config(version,  Some(&config));
                let mut json = serde_json::to_string_pretty(&new_config).unwrap();
                // creates the .kadot file of the sub-version
                PathBuf::push(&mut path, ".kadot");
                let mut output = File::create(&path).unwrap();
                if !(write!(output, "{}", json).is_ok()){
                    println!("Couldn't create .kadot file");
                    return false;
                }

                // updates the config of the parent version
                config.versions.push(version.clone());
                json = serde_json::to_string_pretty(&config).unwrap();
                path.pop(); path.pop();
                PathBuf::push(&mut path, ".kadot");
                if !(write!(config_file, "{}", json).is_ok()){
                    println!("Couldn't create .kadot file");
                    assert!(remove_dir_all(path.as_path().join(version)).is_ok());
                }
                return false;
            },
            _             => return false,
        }
    }
    // if we aren't in a sub-version, it creates a new config and put it into the
    // current directory
    else {
        config = create_config(version, None);
        let json = serde_json::to_string_pretty(&config).unwrap();
        println!("Current path {}", path.to_str().unwrap());
        PathBuf::push(&mut path, ".kadot");
        let mut output = File::create(path).unwrap();
        assert!(write!(output, "{}", json).is_ok());

        return true;
    }
}

fn main() {
    let cli = Cli::parse();

    // println!("Hello, world, {}", cli.name.unwrap_or(String::from("nessun argomento")));
    match &cli.command {
        Commands::Create { version } => {
            create(&version.to_string());
        }
        Commands::Install { version } => {
            println!("Installing version: {version}")
        }
    }
}
