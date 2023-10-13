pub mod cli;

pub mod version;

pub mod config;

pub mod io;

fn main() {
    cli::parse();
}
