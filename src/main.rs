use std::fs;
use std::intrinsics::mir::Len;
use std::process::Command;

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

fn folder_duration(directory_path: &str) -> f64 {
    let mut total_duration: f64 = 0.0;
    let items = fs::read_dir(directory_path).unwrap();

    //println!("{:?}", items);

    for entry in items {
        let path = entry.unwrap().path();
        //println!("{}", path.display());

        // Convert PathBuf to &str for the function call
        let path_str = path.to_str().unwrap();
        let current_duration: f64;
        if path.is_dir() {
            current_duration = folder_duration(path_str);
            println!(
                "{}/ :: {}",
                path.file_name().unwrap().to_str().unwrap(),
                current_duration
            );
            total_duration += current_duration
        } else {
            //If audio video file
            if check_file(path_str) {
                current_duration = get_duration(path_str);
                println!(
                    "{} :: {}",
                    path.file_name().unwrap().to_str().unwrap(),
                    current_duration
                );
                total_duration += current_duration
            }
        }
    }

    total_duration
}

fn check_file(file_path_str: &str) -> bool {
    //println!("{:?}", infer::get_from_path(file_path_str).unwrap());
    match infer::get_from_path(file_path_str) {
        Ok(Some(info)) => {
            //println!("{:?}", info);
            //println!("mime type: {}", info.mime_type());
            //println!("extension: {}", info.extension());

            let mime_type: Vec<&str> = info.mime_type().split("/").collect();

            if mime_type[0] == "video" || mime_type[0] == "audio" {
                true
            } else {
                false
            }
        }
        Ok(None) => {
            //eprintln!("Unknown file type 😞");
            //eprintln!("If you think infer should be able to recognize this file type open an issue on GitHub!");
            false
        }
        Err(_) => {
            //eprintln!("Looks like something went wrong 😔");
            //eprintln!("{}", e);
            false
        }
    }
}

fn get_duration(file_path_str: &str) -> f64 {
    //Try ffprobe in the Shell
    let output_ffprobe = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(file_path_str)
        .output()
        .expect("Error running ffprobe");

    if !output_ffprobe.status.success() {
        println!("Command executed with failing error code");
        return 0.0;
    } else {
        let duration_str = String::from_utf8_lossy(&output_ffprobe.stdout);

        let duration = duration_str.trim();

        if duration == "N/A" {
            println!("Parsed duration: {}", duration);
            return 0.0;
        } else {
            match duration.parse::<f64>() {
                Ok(duration) => {
                    // Parsing successful, use the `duration` value
                    //println!("Parsed duration: {}", duration);
                    return duration;
                }
                Err(err) => {
                    // Handle the parsing error
                    println!("Error parsing duration: {}", err);
                    return 0.0;
                }
            }
        }
    }
}
