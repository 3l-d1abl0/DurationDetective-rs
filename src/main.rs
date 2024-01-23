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
            if stat.is_dir() {
                return true;
            }
            println!("{}", "ERR: Expected path to folder".red().bold());
            false
        }
        Err(err) => {
            println!("ERR: {}", err);
            false
        }
    }
}

fn main() {
    let cli = Cli::parse();

    //println!("path: {:?}", cli.path);
    //println!("write: {:?}", cli.write);

    //Extract the trimmed string
    let path = cli.path.trim();

    println!("Path recieved: {}", path);

    if !is_valid_path(&path) {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::ValueValidation,
            "Please provide a valid fileSystem Path",
        )
        .exit();
    }

    //Check for Write: TODO

    println!("Directory to Scan: {} ", path.cyan());

    println!("Total Folder Duration: {}", folder_duration(&path));
}

fn folder_duration(directory_path: &str) -> f32 {
    let items = fs::read_dir(directory_path).unwrap();

    //println!("{:?}", items);

    for entry in items {
        let path = entry.unwrap().path();
        println!("{}", path.display());

        if path.is_dir() {
            // Convert PathBuf to &str for the function call
            let path_str = path.to_str().unwrap();
            folder_duration(path_str);
        }
    }

    1.0
}
