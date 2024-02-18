use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

use colored::Colorize;

mod cli;
use cli::Cli;

mod utils;
use utils::folder_duration::folder_duration;
use utils::format_seconds::format_seconds;
use utils::is_valid_path::is_valid_path;

fn main() {
    let cli = Cli::parse();

    //println!("path: {:?}", cli.path);
    //println!("write: {:?}", cli.write);

    //Extract the trimmed string
    let path = cli.get_path().trim();

    println!("Path recieved: {}", path);

    if !is_valid_path(path) {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::ValueValidation,
            "Please provide a valid fileSystem Path",
        )
        .exit();
    }

    //Check for Write: TODO

    println!("Directory to Scan: {} ", path.cyan());

    println!(
        "Total Folder Duration: {}",
        format_seconds(folder_duration(path, 0))
    );
}
