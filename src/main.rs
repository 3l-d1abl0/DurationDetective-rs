//use clap::Parser;
use std::fs;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "DurationDetective")]
#[command(author = "Sameer sameer.barha12@gmail.com")]
#[command(version = "1.0")]
#[command(
    about = "A tree like tool for media files",
    long_about = "Generate tree like structure, specifing the duration of media files in each directory !"
)]
struct Cli {
    #[arg(short, long)]
    path: String,
    #[arg(short, long, default_value_t = 0)]
    write: usize,
}

fn is_valid_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    match fs::metadata(path) {
        Ok(stat) => {
            println!("{:?}", stat);
            true
        }
        Err(err) => {
            println!("ERR: {}", err);
            false
        }
    }
}

fn main() {
    let cli = Cli::parse();

    println!("path: {:?}", cli.path);
    println!("write: {:?}", cli.write);

    //Extract the trimmed string
    let path = cli.path.trim();

    println!("PATH: {}", path);

    if !is_valid_path(&path) {
        println!("{}: not a valid path", path);

        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::ValueValidation,
            "Please provide a valid fileSystem Path",
        )
        .exit();
    }

    //Check for Write: TODO
}
