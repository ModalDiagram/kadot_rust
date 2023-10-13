use clap::{Parser,Subcommand};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    name: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        version: String,
    },
    Install {
        version: Option<String>,
    },
}

pub fn parse() {
    let cli = Cli::parse();

    // println!("Hello, world, {}", cli.name.unwrap_or(String::from("nessun argomento")));
    match &cli.command {
        Commands::Create { version } => {
            super::version::create(&version);
        }
        Commands::Install { version } => {
            super::version::install(version);
        }
    }
}
